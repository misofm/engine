# builtins-compiler: fader automation's held-ramp obligation, jump lengths and input arrival are bools, correlated Options and a placeholder

Tracker #2258, item GR-9. Patterns: several bools whose combinations encode one phase, three
`Option`s documented as "`None` exactly when" each other, and a field that holds a placeholder
`0` until a later install writes the real value, where `0` is also a valid value.

## Mission
Model the stored fader automation's per-channel hold phase, per-block jump lengths and per-lane
input arrival `a(n)` so that the illegal combinations cannot be represented and the arrival is
never observable before it is known, without moving a rendered bit.

## Verified (base `5c71b3e8d`)
All in `crates/builtins-compiler/src/lib.rs` unless named.

- **`HeldRampObligation { owed, due_at_entry, held_through }`** (1210-1225): three bools, 8
  combinations. Writers: `FaderCellCursor::rehold` (2193-2234), `HeldRampObligation::block`
  (2758-2791), `AutomatedFaderCell::offsets_jump` (1826), `FaderEventDriver::apply` (3097-3135,
  `owed` only).
  - `due_at_entry` is set `true` only together with `held_through = true` (2230-2231,
    2771-2772), and both are reset by `block` (`mem::take`) and by `rehold`.
  - `offsets_jump` clears `held_through` alone (1826). It runs in the block drain, at the entry
    of the block whose first sample is the one a `due_at_entry` hold frees, so
    `due_at_entry && !held_through` **is reachable**. It is indistinguishable from
    `due_at_entry && held_through`: `block` never reads `held_through` when `due` is set (the
    held arms read neither, `due && owed` does not read it, and the free arm computes
    `held_through && !due`).
  - So the two hold bools carry three phases (free / held through the block's last sample / held
    ending exactly on it), one of which has two spellings; `owed` is independent of them (a
    remembered event survives a ramp that ends muted, constraint 4). Six distinct states, eight
    representable.
- **`fader_jump: Option<u32>`, `mute_jump: Option<u32>`, `vca_jumps: Option<[u32; 2]>`**
  (1266-1277): all three are written together from one `Option<BlockJumps>` in
  `take_block_time` (1456-1461), and documented as `None` exactly when each other is. The event
  driver re-joins them with a four-way `let (Some, Some, Some, Some) = ... else continue`
  (2415-2418); `mute_terms` reads only `mute_jump` as the presence of all three (1499).
- **`arrival: u64`** (1259-1262): `LaneFaderAutomation::new` sets `0` (1325), "`0` until"
  `install_builtin_input_arrivals` writes the member's `a(n)` after lowering
  (`graph-compiler/src/compile.rs` 1282-1294 → `graph::PreparedGraphPlan::install_builtin_input_arrivals`
  (`crates/graph/src/lib.rs` 2865) → `FaderBankProcessor::set_input_arrivals` (5326); the scalar
  oracle's `set_input_arrival` (9399, 9639)). `0` is also the genuine `a(n)` of a member absent
  from `input_arrivals`. Readers: `take_block_time` (`node_span(a(n))`, render), `adopt`
  (`carried_arrival != arrival`, the swap block), and the test probe.
  - **No defect is observable today.** The lanes are built inside
    `into_graph_artifact_with_banks` (7486; `bound_lane_automation` 7135, 7637), and the one
    production caller, `attach_builtins`, installs the arrivals on the next line, before bind and
    before any render or adoption reads the field. No reader branches on `0` as "not installed".
  - What the placeholder does allow: any lowering that does not make the second call (today the
    builtins-compiler unit tests and `graph-compiler/tests/scale.rs`, which call
    `into_graph_artifact_with_banks` directly) silently renders every lane at `a(n) = 0`, and
    the type cannot say whether the value was installed.

## Decision / model
- `HeldRampObligation { owed: bool, hold: HoldAtBlockEnd }`, `enum HoldAtBlockEnd { Free,
  HeldThrough, EndsAtBlockEnd }`: `EndsAtBlockEnd` is "held through the block's last sample and
  free from the next block's first sample, the owed move is due there" (the old
  `due_at_entry`, which implied `held_through`). `offsets_jump`'s release turns `HeldThrough`
  into `Free` and leaves `EndsAtBlockEnd` as it is, which is what the base computed (its cleared
  `held_through` was never read while `due_at_entry` was set). `owed` stays a bool: it is
  orthogonal to the phase.
- `jumps: Option<BlockJumps>` replaces the three Options; the driver destructures one
  `Some(BlockJumps { .. })`.
- `a(n)` is a lowering input, not a later install: `into_graph_artifact_with_banks` (and the
  oracle's `into_graph_artifact`) take the compile report's `input_arrivals`, and every lane is
  built with its member's `a(n)` (`graph::floored_input_arrival`, the same lookup). The
  two-phase install is deleted: `PreparedBuiltinsGraphArtifact::install_builtin_input_arrivals`,
  `graph::PreparedGraphPlan::install_builtin_input_arrivals`,
  `GraphPreparedBuiltinBankProcessor::set_input_arrivals`, and
  `GraphRuntimeProcessor::set_input_arrival` / `GraphNodeBinding::set_input_arrival`. A caller
  that has no report passes `&[]`, which is the report's own encoding of "every arrival is 0",
  not a placeholder.
- SIMD lane words are untouched.

## Non-goals
- No change to `crates/builtins/src/lib.rs` or the graph fixtures (#2354 / PR #2360 own them).
- No change to the automation rules (D1.6, R17-8, R17-10, constraint 4/7) or to any event the
  driver emits.

## Gates
- No observable defect, so no structural red-on-base test is added (test-value rule). The
  refactor is covered by tests shown by mutation: each changed transition reverted or perturbed
  turns a named test red. Attempt 2 folds the two assertions attempt 1's review found missing
  into existing tests (they pin the base rule, so they are green on base).
- One-time no-bit-moved comparison against base (PR evidence, not a committed test).
- `cargo test --locked` for builtins-compiler, graph, graph-compiler, host-core (with
  `test-support` where the suites gate on it); clippy `--all-targets -D warnings` on the four;
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`;
  `scripts/check-workspace-policy.sh`; `realtime_policy`.

## Evidence (attempt 1)
Terra, base `5c71b3e8d`, implementation `9e54f821a`.

**No red-on-base test.** None of the three encodings has an observable defect on base (see
Verified), so no structural test was added. The existing suites cover the change: each mutation
below was applied alone to the new code, and the named tests (host-core `stored_*_automation`,
`successor_swap`, builtins-compiler lib) turned red.

| Mutation | Red |
|---|---|
| m1: R17-10 release never fires (`entry == HeldThrough` → `false`) | `a_live_write_that_frees_a_held_lane_releases_its_hold`, `a_live_write_that_frees_a_lane_with_nothing_owed_releases_its_hold`, `a_hand_mute_inside_a_pending_deferral_lands_the_lane_on_the_new_table_at_the_unmute` |
| m2: `jumped_at_entry` does nothing | `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp` |
| m3: `jumped_at_entry` also clears `EndsAtBlockEnd` | **survives** (see below) |
| m4: `rehold` never yields `EndsAtBlockEnd` | `a_hold_that_starts_and_ends_inside_a_block_moves_at_its_end` |
| m5: `block` never yields `EndsAtBlockEnd` | 7 tests, for example `a_remembered_jump_or_seek_set_moves_the_lane_at_the_unmute_ramps_end` and `the_move_at_the_unmute_ramps_end_replaces_the_cells_event_there` |
| m6: the due move ignores `owed` | **survives**. Base logic, not this change: a cut at 0 with nothing owed moves nothing, so the mutation looks equivalent |
| m7: bank lanes built at `a(n) = 0` | `each_automated_fader_lane_takes_its_arrival_and_node_time_per_block`, `a_fader_step_meets_its_source_frame_through_a_latent_insert`, `a_vca_ride_reaches_each_member_in_its_own_node_time` and 5 more |
| m8: oracle lanes built at `a(n) = 0` | `the_scalar_oracle_renders_the_banks_fader_events`, `a_session_seek_sets_a_latent_fader_lane_exactly_where_it_reaches_the_node` and 4 more |
| m9: fader and mute row lengths swapped in the driver | 19 tests across the fader, mute and VCA suites |

m3 is the one reachable state the old bools spelled twice (`due_at_entry && !held_through`). It
needs an owed move that is due at a block's entry plus an offsets jump applied at that same entry.
No existing test builds that case. This change keeps base behaviour there, so nothing moves. The
question of whether base's "the due move still lands over the jump" is the right rule is a
follow-up, not part of this item.

**No bit moved.** This was a one-time comparison, not a committed test. A temporary patch to
`engine::PreparedRenderPlan::render` (scratchpad `gr9/dump.patch`, never committed) appended the
FNV-1a hash of every rendered output block to a per-test file. It ran over host-core
`stored_fader_automation`, `stored_mute_automation`, `stored_vca_automation`, `successor_swap`
and `session_seek` (with `test-support`), once on base and once on the branch:

- 118 dump files and about 61.9k block hashes. Every file is identical except one.
- The exception is `session_seek`'s single concurrent test, whose block count depends on timing
  (21,291, 21,294 and 21,296 blocks on three runs). Its common prefix is identical.
- Pass/fail verdicts were identical on both sides. The dump allocates, so the
  allocation-counting tests fail on both sides, at the same assertions.

**Plan bytes.** Rendered output does not change, but the resource estimate does:
`size_of::<LaneFaderAutomation>()` goes from 416 to 408 bytes and `HeldRampObligation` from 3 to
2 bytes. `fader_automation_sizes().lane_slot_bytes` and the unbanked-owner charge both read
`size_of`, so a plan with stored fader automation reports 8 fewer bytes for each automation lane
slot. No committed fixture pins those bytes, and the graph fixtures are not touched.

**Gates (this worktree, own target dir):**
- `cargo test --locked` passes for the four crates:
  - `-p builtins-compiler --all-targets` with `builtins-compiler,graph,engine,session/test-support,engine/realtime-audit`
  - `-p graph -p graph-compiler --all-targets` with the same features
  - `-p host-core --all-targets` with `host-core/test-support` added (471 tests)
- `cargo clippy --locked -p builtins-compiler -p graph -p graph-compiler -p host-core --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --all --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins-compiler -p graph -p graph-compiler`: clean. A per-package host-core doc fails on an existing link in
  `host-core/src/prepare.rs` 262 (`crate::warm::production_config`). That file is untouched, and
  the failure is not caused by this change.
- `scripts/check-workspace-policy.sh`: ok.
- `realtime_policy`: ok (298 marked regions in 48 files).

## Verdict (attempt 1)
Sol: **FAIL** at head `cefd78dec`, on two MAJORs and one MINOR. The refactor itself held up:
- Rendered bits are identical to base.
- The workspace builds on the merged tree, and fixtures and budgets hold.
- m6 is equivalent.

1. **MAJOR: m3 changes rendered bits, and the new `jumped_at_entry` transition has no test.**
   - The reviewer's probe: track 0 is prepared muted on a curve flat at -30 dB to 300, then
     fading to 0 dB at 10,000. A live unmute at 0 ends its ramp exactly on a block's last
     sample, then an offsets write `S = -6` dB (ramp 0, 1 or 200) lands at the next entry.
   - Base and the branch agree. Under m3, every case whose window ends on a block's last sample
     differs, at quanta 128, 64 and 37. At q128 with ramp 0, the first sample is 0.020517617
     against 0.020516941.
2. **MAJOR: the per-member arrival lookup in `lower_strip_banks` is untested.** Mutant f4 gives
   every lane `members[0]`'s arrival and survives everything. The reason:
   `each_automated_fader_lane_takes_its_arrival_and_node_time_per_block` claims to cover another
   member's arrival, but its two tracks land in different banks.
3. **MINOR:** mutants f2 and f3 are recorded as follow-ups below.

## Evidence (attempt 2)
Terra, merged origin/main `2aa2aa60c` (#2354's builtins change and its regenerated fixtures) at
`465f92653`. Tests are at `5207c3cdd`. The rule is unchanged.

**The two folded assertions** (host-core `stored_fader_automation.rs`):
- **Contrast case** in `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp`, at
  quanta 128, 64 and 37 on every render form:
  - Setup: a live unmute drained at 0 over a window `W` (the first multiple of the quantum at or
    after 1,200). Its hold ends on a block's last sample, over the fade's grid events from 1,100,
    so a move is owed and due at `W`. An offsets write `S = -6` dB lands at `W` with ramp 1 or
    480.
  - Assertion: from `W` on, the two outputs are bit-identical. The due move lands over the
    fader length and replaces the jump's ramp.
  - Test value: red if an offsets jump at an entry where an owed held ramp ended on the previous
    block's last sample cancels the due move. It is red under m3: `quantum 128: plane 0`
    diverges at 1,280.
  - A one-time probe on the merged tree showed the same split the reviewer saw. Ramps 1 and 200
    are identical from the entry. Ramp 0 differs, because its jump lands before the move. This
    is why the test compares two nonzero ramps.
- **Shared bank** in `each_automated_fader_lane_takes_its_arrival_and_node_time_per_block`:
  - Track 1 gains a latency-free `miso.compressor` insert, which puts its fader at track 0's
    dependency level, so both faders are lanes 0 and 1 of one bank. The test asserts this.
  - The arrivals assertion now separates the members. Under f4 it is red: `[486, 486]` against
    `[0, 486]`.

**Mutants re-run** on the merged tree, each alone (host-core `stored_*_automation`,
`successor_swap`, builtins-compiler lib):

| Mutant | Tests red |
|---|---|
| m3 | 1 (the contrast case above) |
| f4 | 1 (the arrival test) |
| m1 | 3 |
| m2 | 1 |
| m4 | 1 |
| m5 | 8 (now including the contrast case) |
| m6 | 0 (equivalent, as attempt 1's review agreed) |
| m7 | 8 |
| m8 | 6 |
| m9 | 19 |

**No bit moved on the merged tree.** Same method as attempt 1, with one difference: the base
side is `2aa2aa60c`'s builtins-compiler, graph and graph-compiler under the branch's host-core
tests, so both sides run the same tests.
- 118 dump files and about 63.2k block hashes. Everything is identical except `session_seek`'s
  timing-dependent concurrent test (21,279 against 21,293 blocks), whose common prefix is
  identical.
- Pass/fail verdicts are identical on both sides.

**Gates on the merged tree:**
- `cargo test --locked` passes for each crate run alone, with test-support features:
  - builtins-compiler: 58 tests
  - graph: 150
  - graph-compiler: 194
  - host-core: 471
- clippy `--all-targets --all-features -D warnings` on the four crates: clean.
- `cargo fmt --all --check`: clean.
- rustdoc `-D warnings` for builtins-compiler, graph and graph-compiler: clean. The host-core
  `prepare.rs:262` link failure already exists on main.
- Workspace policy: ok. `realtime_policy`: ok.

## Verdict (attempt 2)
Sol: **FAIL** at head `585be9c5b`, on one MAJOR. The m3 contrast case was judged sound.
- **MAJOR: f4's gap was only half closed.** The arrival test sorted the arrivals before comparing
  them, so it checked which arrivals were present, not which lane held which. Mutant f5 reverses
  the arrivals inside the bank, so track 0, behind the latent limiter, renders at `a(n) = 0`.
  f5 survived host-core, builtins-compiler and graph-compiler.
- **MINOR:** pin the contrast case absolutely as well, and record follow-ups n1, n3 and n8.

## Evidence (attempt 3)
Terra. The branch merges origin/main `4ba877b3b` at `aeefcfd89`. Tests are at `01028f714`. The
rule is unchanged.

**What changed in the tests:**
- **Arrival test.** `each_automated_fader_lane_takes_its_arrival_and_node_time_per_block` no
  longer sorts. It takes the fader bank's lane-to-track map from the plan: the one banked unit
  that does not gather its tracks' input and renders both automated tracks
  (`PlanUnitEligibility::lane_tracks`). It then requires each probed lane's `arrival` to equal
  that lane's own track's `a(n)`. Test value: red if a fader-bank lane takes another member's
  `a(n)`, in any order.
- **Contrast case.** The case in `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp`
  now also requires the ramp-1 output to equal the never-muted session that takes the same
  -6 dB write at `W`. The comparison starts at the first grid completion sample `64k + 63` at or
  after the move's end `W + 480`: 1,791, 1,727 and 1,727 at quanta 128, 64 and 37.
  - A one-time probe placed the last differing sample at 1,790, 1,726 and 1,726, one sample
    before each bound. So the bound is exact, not padded.
  - What this assertion pins is convergence: once the move and the grid's next retarget are
    done, the lane is on the never-muted composed curve, coincident -6 dB write included. It does
    not pin the due move or its target. m3 and a "due move never fires" mutant both pass it on
    their own; the ramp comparison is what pins the move.

**Mutants (merged tree):**
Each mutant was applied alone. The suites run were host-core `stored_*_automation` and
`successor_swap`, plus the builtins-compiler lib.

| Mutant | Red | Assertion that fails |
|---|---|---|
| f4: every lane takes `members[0]`'s arrival | the arrival test | `block 0, lane 1: the lane's own track's a(n)` (`Some(486)` against `Some(0)`) |
| f5: `members[members.len() - 1 - lane]` | the arrival test | `block 0, lane 0: the lane's own track's a(n)` (`Some(0)` against `Some(486)`) |
| f6: off by one, `members[(lane + 1) % members.len()]` | the arrival test | `Some(0)` against `Some(486)` |
| m3 | the contrast case | `quantum 128: plane 0` diverges at 1,280 |

Attempt 2's runs of m1, m2, m4-m9 do not depend on the two edited assertions. Every one of those
mutants that was red is caught by some other test besides these two.

**Gates on the merged tree:**
- `cargo test --locked` passes for each crate run alone, with test-support features:
  - builtins-compiler: 58 tests
  - graph-compiler: 194
  - host-core: 471
- clippy `--all-targets --all-features -D warnings` on builtins-compiler, graph, graph-compiler
  and host-core: clean.
- `cargo fmt --all --check`: clean.
- Workspace policy: ok.
- The attempt-3 commits change only host-core tests and this spec, so render code is untouched
  and attempt 2's no-bit-moved comparison stands.

## Verdict (attempt 3)
Sol: **PASS** at head `58115e249`, with two MINORs. Both are folded in after the verdict.
- **The move length and grid are derived, not hard-coded.** The contrast case takes the move's
  length from `controls::LiveRamps::for_session(&model).fader_samples`, as the arrival test does,
  and writes the bound as `(w + move_len) | (GRID - 1)` with a local `const GRID: usize = 64`.
- **The convergence assertion is described accurately.** The test's comment and doc, and the
  attempt-3 evidence above, now say it pins convergence to the never-muted composed curve, not
  the due move's target.
- **The optional target pin was tried and dropped.** It compared the move's completion sample
  `W + 479` with `checked_fader_gain(vca_compose_db(curve(W + 479), S))`, and that did not hold
  (q128, plane 0: bits `1017889684` against `1017621431`). The landing targets the curve at the
  jump's completion through the ramp kernel, not that sample's composed value. Pinning the target
  exactly would need the kernel's own arithmetic, so it is left out rather than approximated.
- After the folds, host-core `--test stored_fader_automation` (test-support: 35 tests), clippy
  `-D warnings` and fmt pass.

## Follow-ups (not this issue)
- **Owner ruling needed.** On base, a move that comes due at an entry (`EndsAtBlockEnd`, owed)
  still lands over an offsets jump applied at that same entry, and cancels the jump's ramp.
  That sits oddly with the F1 rule, which says a jump's hold is newer than a ramp a live write
  ended. The contrast case pins the current rule until an owner decides.
- **f2:** the R17-10 release also fires on `EndsAtBlockEnd`. This is probably equivalent: a hold
  that ended by itself has no cell hold left to release.
- **f3:** the cursor's muted `rehold` gives `EndsAtBlockEnd`. That arm is untested on base too.
- **n1:** `rehold` returns `EndsAtBlockEnd` for any hold that ends past the block. No test covers
  a curve unmute whose ramp crosses a block boundary, followed by a live write that frees the lane
  at the next entry. This is base behaviour.
- **n3:** the due arm of `block()` also releases the cell's hold. This is probably equivalent.
- **n8:** `mute_terms` ignores whether jumps are present. This is probably equivalent.
