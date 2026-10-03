# #1242 attempt 1 verdict: Apply VCA offsets and mutes at preparation

**Verdict: FAIL.** There is one MAJOR: a both-lanes kind 4 (mute) in the browser stages the wrong
value on one lane when a VCA mutes only the other. That is a live mute defect this slice
introduces, and it breaks the umbrella's binding "mute wins" rule. There is also one MINOR (an
undefended seeding path) and two NITs. Everything else holds up: the preparation path (both
compilers, the C ABI included), the composition, the domain check, the phase-two observation,
the bit identity of sessions without VCAs, and every spec gate.

- **Implementation:** `250b72e94` on parent `7f106a6de`, branch `codex/batch-vca`.
- **Review copy:** `git archive` exports of `250b72e94` and `7f106a6de` under `/tmp/claude-1002/v1242/`
  (deleted after review). I never touched the worktree, which #1241 is editing concurrently.
- **Host:** x86-64-v3 AVX2, AMD EPYC 7313P.
- **Probes:** `1242-attempt1-verifier-scratch.rs`, next to this file.

## Gates re-run on `250b72e94` (all exit 0)

- **test-debug-a** (CI's command, `--no-fail-fast`): 1,259 passed, 0 failed and 9 ignored, over
  112 binaries. CI's preceding `cargo test --locked -p builtins-compiler --no-run` (default
  features) also exits 0. The run covers every new test and `allocation_tracker.rs`.
- **Gate 8:**
  - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`.
  - `audit capi`: `pcm_digest ff6cdcb96cdcdad5` with 0 violations, identical to `audit capi`
    built from the base export.
  - `check-graph-determinism.sh` PASS 100/100. `target/issue6/fresh-process-determinism.json` is
    byte-identical (`cmp`) to the one the base export produced.
  - `graph_fixture -- --check`, `check-builtins-fixtures.sh` and `check-console-fixtures.sh`.
  - `cargo test --locked --release -p audit -p bench -p console-workload`. Nothing was re-pinned.
- **Gate 9:** not applicable. #1053 has not landed: `live_builtin_delta` appears nowhere in Rust
  source, and every C ABI commit is structural (`commit_prepared_structural`, which prepares
  through `prepare_host_runtime`). The VCA guard is present in #1053 A2 D1 (`:155-160`), in
  #1225's D1 structural list (`:110-113`) and in #1226 D5 (`:75-80`).
- **Gate 10:**
  - Fresh `build-web-audioworklet.sh --named-twin B A` and `check-web-audioworklet.sh`.
  - `check-browser-expected-resources.py --artifacts A`, with no re-pin.
  - `check-sdk-headless.sh A`: 357 pass, 0 fail.
  - `cargo fmt --check`, clippy `-D warnings` and `cargo doc -D warnings`.
  - The session, builtins, graph, host-core, realtime and workspace policy checks and their
    self-tests. `test-workspace-policy.sh` prints the same "directed fault unexpectedly passed"
    lines on the base and exits 0 on both.
  - `check-cross-targets.sh` PASS: host-core is still at its 4-call ceiling, and there is no new
    `memset_pattern16` row.
  - `run-aarch64-tests.sh debug` waits for CI's `aarch64-debug` at the batch push, because this
    host is x86-64.

## MAJOR-1: a both-lanes kind 4 on a strip whose VCA mutes one lane stages the wrong lane

`hosts/host-web/src/lib.rs:4595-4609` (the `COMMAND_MUTE` arm) reads the effective mute of the
**first covered lane only** and stages one record for every covered lane:

```rust
// The first lane the selector covers: after `set_user_mute` every covered lane
// holds `muted`, and the solo term is per strip, so the covered lanes agree.
let lane = usize::from(matches!(lanes, BuiltinLaneSelector::Right));
let effective = ready.solo.effective_mute(track, lane);
ready.solo.record_emitted(track, lanes, effective);
staged[0] = AdmittedCommand::Fader(TrackFaderRecord::Mute { lanes, muted: effective, .. });
```

Before this slice, "the covered lanes agree" was true. D3 adds a **per-lane** `vca_mute` term to
`effective_mute`, which makes it false: with `Both`, the two lanes now differ whenever a reaching
VCA mutes one lane only. A per-lane VCA mute is ordinary schema (`fader.left_mute` and
`right_mute`), and gate 6's own fixture uses one.

- **The VCA mutes the right lane; kind 4 `channel` 2 (both) `false` on the member.** The record
  is `Both, false`, so the **VCA-muted right lane opens**. That breaks umbrella rule 5 ("mute wins:
  solo never clears a user or VCA mute"), the slice's product outcome, and the sentence
  `BUILTINS_AND_METERING_V1.md` gains in this commit: "an explicit unmute of a VCA-muted member
  records the intent but stages the lane still muted". `emitted` becomes `[false, false]` against
  an effective `[false, true]`.
- **The VCA mutes the left lane; the same command.** The record is `Both, true`, so the right
  lane goes silent, though nothing mutes it. `emitted` becomes `[true, true]` against an effective
  `[true, false]`. The follow pass reads the true per-lane effective mute and leaves the
  following send's right column open. The strip's right lane is then muted while its follow send
  stays open: the live mirror and the render plane disagree.
- The coalescing pass runs only `if solo_seen`, so nothing repairs the record before the next
  solo toggle.

**Evidence.**

- P1, `sol_probe_both_lane_unmute_on_a_one_lane_vca_mute`, on #1224's follow session with VCA
  `fx` muting one lane of `drums` and smoothing 0: red in both cases. The first difference against
  an untouched host is at block 1, sample 128 (the right plane), in each case.
- P2, `sol_probe_browser_vca_equals_written_directly_under_solo_and_mute`, is a randomized browser
  differential:
  - It builds random VCA forests over the five strips, with offsets across the domain and drawn
    lane mutes. Each seed then runs eight random batches of solo toggles and kind 4 edits, every
    one at smoothing 0.
  - The VCA host is compared every block against a host booted with no VCA and every fader
    written as its effective value, computed independently. That host gets the same solos, and for
    each kind 4 the per-lane equivalent `value || vca_mute[l]`.
  - Result: **146 of 1,500 seeds red**, each ending in a `channel` 2 kind 4 on a strip with a
    one-lane VCA mute. Solo-only (`SOL_ONLY_SOLO=1`): **0 of 1,500 red**.

**Fix (verified in the scratch copy).**

1. When `lanes` is `Both` and `effective_mute(track, 0) != effective_mute(track, 1)`, stage two
   records, `Left` with the left value and `Right` with the right, and return `produced = 2`. The
   `staged` array already has two slots, and `command_staging_count` already budgets
   `MAXIMUM_COMMAND_RECORDS * 2`.
2. `record_emitted` each lane separately.
3. Otherwise stage one record, exactly as today, so nothing moves for a session without VCAs.
4. Rewrite the "covered lanes agree" comment.

With that patch, P1 is green, P2 is green at 1,500 seeds and 8 batches each, and the whole
`host-web` suite still passes (169 tests). Commit P1 as the regression test: it is red on the
revert. Commit P2 at 48 seeds as well (1.3 s in debug), which closes MINOR-1 at the same time.

## MINOR-1: no committed test defends host-web's submix VCA-mute seeding

- **Mutation M2:** host-web seeds `vca_mute: [false; 2]` for submixes only and leaves the tracks
  alone.
- It stays **green across the whole suite** of `session`, `builtins-compiler`, `graph-compiler`,
  `host-core` and `host-web` (all test-support features).
- The defect it plants is real: the solo state believes a VCA-muted bus is open while the plan
  has it muted, so a later kind 4 un-mute of that bus reopens it (P2 seed 6: a single-lane
  un-mute of `room`).
- Gate 5's browser test VCA-mutes `room`, but never issues a kind 4 to it, and nothing in it
  follows `room`.
- **Fix:** with MAJOR-1 fixed, P2 turns red on M2 (12 of 48 seeds). The minimal alternative is to
  extend `a_vca_muted_member_stays_muted_through_solo_and_an_explicit_unmute` with a kind 4
  `false` on `room` (strip 3) before the final lockstep.

## NITs

- **NIT-1.** `compile_ready` calls `model.effective_strip_faders()`, which allocates without
  `try_reserve`, between neighbours that map allocation failure to `web.resource.allocation`.
  The allocation is control-plane only, bounded by the strip count and freed before boot ends,
  and the compilers allocate the same way. Optional.
- **NIT-2.** One browser preparation runs the composition up to four times: the tail seal, the
  lowering, the graph follow map and the host-web seeding. Each run costs O(strips x reach).
  Harmless until #1243's cap. Do not restructure it in this slice.

## What holds up under probing

- **The composition (D1).**
  - `vca_effective_db` sums in `f64`: the member first, then the offsets in the order given. It
    clamps once, rounds once with `as f32`, and returns `member_db` bit for bit when there is no
    offset.
  - `vca_reach` walks upward over a member-to-parents map with a visited set that is reset for
    each strip, and sorts by VCA ID. A diamond counts once, and the walk terminates on cycles.
  - The graph compiler and host-web both index `effective_strip_faders()` in `strips()` order of
    the **normalized** model. Host-web's `effective_faders[prepared_mutes.len()]` matches its
    tracks-then-submixes loop.
- **The floor and -inf.** A VCA or VCA sum that reaches -144 dB acts as a fader at -144 dB, not as
  a mute. Probe `sol_probe_vca_floor_is_a_fader_minimum_not_a_mute`: 0 + (-144) + (-144) clamps
  to -144, and so does -100 + (-144) + 24. The render equals a plain fader at -144 bit for bit and
  is not all zero, which is the spec's clamp ruling.
  - `-inf` and NaN cannot arise:
    - VCA offsets are validated finite in `[-144, 24]` (#1240).
    - A non-finite own fader is refused by the own-value preflight.
    - The sum of finite values clamped to the domain is always in the domain, so the lowering's
      `expect("preflighted parameters")` cannot fire on an accepted session.
- **Two consumers, one computation (D2).**
  - The preflight uses `own_fader` (gate 7). Lowering and `expected_tails` use the effective
    values. Controls are built from the lowered `parameters`, so a live ramp starts from the
    effective value.
  - `effective_faders` is computed before `TestPhaseTwoAllocationGuard::begin()` and dropped after
    the guard. My M8, which moves it inside the guard, turns the existing
    `phase_two_allocator_layouts_match_the_checked_resource_report` red, so the observation is
    defended.
- **The mute state (D3).**
  - `emitted` is seeded with `mutes || vca_mute` and `user_mute` with `mutes`.
  - `effective_mute = user || vca || solo term`. A VCA mute never touches `solo_count`.
  - The solo state is built before `LiveRouteState::try_new`, which reads `solo.effective_mute`.
  - P2 with solo only (1,500 seeds x 8 batches): solo composition, solo-safe buses, follow
    mirrors and the seeding all render bit-identically to the written-directly host.
- **Prepared-path randomized differential.**
  `sol_probe_vca_forest_with_sends_and_follows_equals_written_directly` ran 96 seeds x 40 =
  **3,840 seeds** in release, all green.
  - Generator: nested forests (up to 8 VCAs over 4 levels, diamonds, up to 16 members, tracks
    and submixes), 2-5 tracks and 1-4 submixes.
  - Routes: random sends on every non-insert tap, track to bus and bus to bus, with random
    `follows_mute`, route mute, gain and matrix. Own faders and offsets are drawn across the
    domain.
  - Check: the whole output is bit-identical to the session written directly from an independent
    top-down reach and `f64` reference. `effective_strip_faders()` also equals that reference bit
    for bit.
  - Reach: 6,301 VCA-muted submixes, 7,214 VCA-muted follow sources (1,552 of them submixes), and
    3,015 seeds with a clamped lane.
- **No bit moves without a VCA.** By construction, an empty reach gives the own bits and the own
  mutes, and host-web seeds `[false; 2]`. Gate 8's digests and determinism JSON confirm it
  against the base.
- **Deviations accepted.**
  - The out-of-path `model.rs` edit is two doc comments that would otherwise say "inert until
    #1242".
  - Gate 5's browser half uses `follow_lockstep`, because `feed_and_render` cannot carry a member
    claim (DESIGN section 7, VERIFY-2 M13).
  - Gate 3 is split between a sealed-text half and a render half.
  - D4 is correctly recorded as not applicable.

## Test value (one sentence each)

- `vca_effective_db_sums_in_f64_in_order_and_clamps_once`: turns red if the sum accumulates in
  `f32`, runs in another order or clamps at intermediate sums (my M7: red).
- `vca_reach_counts_a_diamond_once_and_includes_nested_parents_in_id_order`: turns red if reach
  double-counts a diamond, misses a nested parent, is left in declaration order, or leaks across
  strips (my M5, visited set not reset: red).
- `vca_reach_terminates_on_a_cyclic_model`: turns red if the upward walk has no visited set, so
  it hangs on an unvalidated cyclic model.
- `a_vca_member_fader_outside_its_domain_refuses_at_its_own_path`: turns red if the preflight
  checks the clamped effective value.
- `a_vca_muted_member_seals_its_follow_zeroed_rows`: turns red if route lowering reads the own
  mute or reaches tracks only (my M1, follow map reading `vca_mute` only: red, together with
  #1218's tests).
- `a_vca_forest_renders_the_bits_of_its_effective_faders`: turns red if an offset reaches the
  wrong strip or lane, misses a nested or submix member, or is summed differently (M5 and M7: red).
- `a_post_fader_send_follows_the_vca_and_a_vca_mute_silences_a_follow_send`: turns red if the VCA
  lands after the sends or a fresh plan leaks a VCA-muted pre-fader follow send (my M6, lowering
  bakes the own mute: red).
- `a_vca_mutes_a_submix_member`: turns red if a VCA mute reaches tracks only (M6: red).
- `a_vca_mute_composes_under_every_solo_precedence_rule`: turns red if solo clears a VCA mute,
  solo-safe exempts a strip, a VCA mute counts toward `any_solo`, or `emitted` is seeded without
  it.
- `a_vca_muted_member_stays_muted_through_solo_and_an_explicit_unmute`: turns red if solo or a
  both-lanes un-mute clears a **two-lane** VCA mute on a track (M6: red; my M3, track seed read
  from `.mute`, is red through #1224's test instead). It does not defend the submix seeding
  (MINOR-1) or a one-lane VCA mute (MAJOR-1).
- `a_live_send_edit_keeps_a_vca_muted_column_zeroed`: turns red if the live-send mirror is seeded
  without the VCA mute, so the first send edit reopens a VCA-muted column.

## My mutations (each applied to the scratch copy, run, reverted)

| # | Mutation | Result |
|---|---|---|
| M1 | graph follow map reads `fader.vca_mute` (drops the own mute) | red: `vca_follow`, `route_coefficients`, `route_mute` (3), `live_routes`, `vca.rs` gate 3, 5 host-web #1224 tests |
| M2 | host-web seeds submixes with `vca_mute: [false; 2]` | **green in the whole suite** (MINOR-1); red in P2 (12/48 seeds) |
| M3 | host-web seeds tracks' `vca_mute` from `.mute` | red: `a_one_lane_mute_follows_into_its_own_source_column` |
| M5 | `vca_reach` never resets `visited` between strips | red: gate 1 reach test, gate 2, gate 3 render and text |
| M6 | lowering bakes the effective dB with the **own** mute | red: gates 2, 3, 4 and gate 5 browser |
| M7 | clamp after every partial sum | red: gate 1 composition test, gate 2 |
| M8 | `effective_strip_faders()` inside the phase-two guard | red: `phase_two_allocator_layouts_match_the_checked_resource_report` |

## For attempt 2

Fix MAJOR-1 as described and commit P1. Then either commit P2 (48 seeds) or extend gate 5's
browser test for MINOR-1. Re-run the host-web suite, test-debug-a, clippy and
`check-web-audioworklet.sh`. The kind 4 arm is in the `command_submit` closure, which the worklet
gate audits for traps. Nothing else in the slice needs to change.
