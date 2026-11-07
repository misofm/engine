# Builtins fader: land the lane phase enum with its regenerated plan fixtures and a mute-during-unmute test

## Origin

This is the bounded successor to #2280 (tracker #2258, GR-1). #2280 stopped under the five-attempt rule after attempt 5 failed review on 2026-11-07. Its code is correct and is kept on branch `codex/issue-2280-fader-lane-phase` at `f82131236`:
- the fader lane phase enum `FaderPhase { Open, Muted, Unmuting, Armed{opens} }`, which replaces four correlated bool arrays in `crates/builtins/src/lib.rs`;
- no rendered bit moved: simd4 `c50ab7ba3c148cb5`, simd8 `c632900598d806ce`, scalar `1d4ee1dfcd9290db`, identical on base and head;
- 52 of 66 review mutants killed, and 14 accepted with named reasons.

#2280's spec, `.github/ISSUE_SPECS/2280-builtins-a-fader-lane-s-phase-is-four-parallel-flags-and-a-lane-import-copies-them-unchecked.md` at `f82131236`, records the whole history. #2280 is closed, and that spec was never on `main`, so this issue's branch deletes it. This issue owns exactly the two things attempt 5 lacked, plus the per-channel carry contract that attempt 1's review added (below).

## Scope

1. **Regenerated plan fixtures.**
   - The enum shrinks each builtin bank. For example, `v1/direct-route.resources.json` `builtin_bank_bytes` drops from 7255 to 7223, and `incremental_plan_bytes` / `session_plus_plan_bytes` drop from 75223 to 75191.
   - So `graph-compiler` `graph_fixture::tests::checked_in_fixtures_are_the_generated_bytes` is red on the branch.
   - Regenerate the fixtures with `cargo run -p graph-compiler --bin graph_fixture -- --write` under the pinned AVX2 flags, commit them, and record the per-fixture byte deltas.
2. **The `Unmuting -> Muted` transition gets a test.**
   - Mutant F5 survives: in `mute_channel`'s no-op check (~3481 on the branch), `phase.muted() == muted` becomes `phase.gain_held() == muted`. A mute during an unmute ramp then returns early, and the strip stays audible while the model says muted.
   - Add, inside `lane_state`'s existing tests (no sibling test): unmute over 64 frames, then mute with window 0 and with a window. Assert `is_muted == Some(true)` and an exact `+0.0` block once the mute lands. A mute during an opening fire's fade-in was tested the same way in attempts 1 and 2. Attempt 3 folds it away under "one test per claim": it goes through the same `Unmuting -> Muted` transition and kills no mutant on its own.
   - F5 must go red.
3. **The per-channel lane carry gets a test** (added by attempt 1's review).
   - #2280 rewrote `export_lane` and `import_lane`. No test carries a lane whose two channels differ, so four mutants survive:
     - F8: import takes channel 0's phase.
     - F14: export takes channel 0's phase.
     - G3: import settles each channel against channel 0's `remaining`.
     - G5: export writes channel 0's `remaining` for both channels.
   - The defects behind them:
     - A lane with the right channel muted and the left open comes back right-unmuted. After a timed SetGain it is audible, so a plan swap un-mutes a muted right channel.
     - A lane whose right channel is mid-unmute loses its hold, so a fader move cuts the ramp into a step.
   - The test goes inside `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders`. It has two rows whose channels differ. Each row is exported, imported into another lane, and given the same timed SetGain on both lanes. Then the `run_fader` words are compared, and `is_muted` and `gain_held` are asserted per channel.
   - F8, F14, G3 and G5 must go red.

## Non-goals

- No change to the enum or its transitions unless a gate proves one is wrong.
- The dual-mono channel-index gap outside the lane carry (N12, N15, N16 and F7) is its own issue (#2355).

## Gates

- `cargo test --locked` for `builtins` (test-support), `builtins-compiler` (test-support), `graph-compiler` and `host-core`, one crate at a time. These are the real dependents of `builtins`.
- clippy `-D warnings`, fmt, rustdoc `-D warnings`, `scripts/check-workspace-policy.sh`, `realtime-policy`.
- F5 is red, and the 18 table mutants the #2280 attempt-5 review re-ran are still red. Scripts are in the session scratchpad `review-2280e/scripts/`.
- A no-bit-moved comparison with the `gr1_bits` digests above, recorded as PR evidence.

## Attempt budget

3 attempts. The brief freezes this smaller limit.

## Evidence and attempt record

### Attempt 1

Branch `codex/issue-2354-fader-phase-enum-lands`, cut from `codex/issue-2280-fader-lane-phase` at `f82131236`. `origin/main` at `8147ab6b8` was then merged in with a normal merge, which had no conflicts. The enum and its transitions are unchanged.

**Fixtures.** `cargo run --locked -p graph-compiler --bin graph_fixture -- --write` ran under `.cargo/config.toml`'s pinned `+avx2,+fma` target features, with no `RUSTFLAGS` override. Only one fixture changed:

| fixture | field | before | after | delta |
| --- | --- | --- | --- | --- |
| `v1/direct-route.resources.json` | `builtin_bank_bytes` | 7255 | 7223 | -32 |
| | `incremental_plan_bytes` | 75223 | 75191 | -32 |
| | `session_plus_plan_bytes` | 75223 | 75191 | -32 |

The file stays 435 bytes long. Its `MANIFEST.tsv` sha256 changes from `70838ee9...` to `877f799a...`. Every other graph fixture regenerated byte-identical. After the regeneration, `graph_fixture::tests::checked_in_fixtures_are_the_generated_bytes` is green.

**Test.** The new cases sit inside `lane_state`'s existing tests, with no sibling test.
- `an_unmute_holds_the_gain_only_while_its_countdown_runs`: a lane is muted, then unmuted over 64 frames, then muted again, with window 0 and with window 16. Both channels must read `is_muted == Some(true)`. Once the mute has landed, the next block must be exact `+0.0` (all words `0`). For the windowed case, the block after the ramp block is checked.
- `an_arm_holds_the_lane_until_its_fire_across_a_carry_and_a_reset`: an opening fire runs a 64-frame fade-in, then the lane is muted during the fade-in, with window 0 and with window 16 (re-armed and re-fired first). The test asserts the same two things.

Test value: in `mute_channel`'s no-op check, testing `gain_held()` instead of `muted()` takes a mute during an unmute ramp or a fade-in for a repeat. The strip then stays audible while the model says muted. No earlier test catches this (F5 survived the whole #2280 suite).

**Mutants.** The scripts are scratch-only (`i2354/runm.sh`, which applies `review-2280e/scripts/fresh.py`'s table).
- **F5** (`phase.muted() == muted` becomes `phase.gain_held() == muted`) is **red** in both tests, against `-p builtins --lib`. Failure: `Simd4: a mute over 0 frames during an unmute ramp`, left `Some(false)`, right `Some(true)`. The same failure appears for a fade-in.
- **The 18 table mutants** the #2280 attempt-5 review re-ran are all still **red**, each against `-p builtins --lib`: A, C, D, M2, N10, N17, N21, N24, N26, P4, X12, X4, X9, Y14, Y2, Y7, Y8, Y9. Each is killed by the same `lane_state` test as in that review.
- **Finding: the stale fixture had masked two mutants.** In the #2280 attempt-5 review, fresh mutants F8 (`import_lane` takes the phase from channel 0) and F14 (`export_lane` takes the phase from channel 0) were "killed" only by the stale `checked_in_fixtures_are_the_generated_bytes`, which was red for an unrelated reason. With the fixtures regenerated, both **survive** the full `builtins` (test-support) and `graph-compiler` suites, and so does F7 (#2355's). Attempt 2 moves F8 and F14 into this issue (see the review verdict below).

**No bit moved.** The scratch harness `gr1/gr1_bits.rs` was copied into `crates/builtins/tests/`, run with `cargo test --release -p builtins --features test-support --test gr1_bits`, then removed, so it is not committed. It gives simd4 `c50ab7ba3c148cb5`, simd8 `c632900598d806ce` and scalar `1d4ee1dfcd9290db`, the same as the #2280 base and head.

**Gates.** Each crate was run on its own, on the merged tree.
- `cargo test --locked -p graph-compiler`: 206 passed, 0 failed.
- `cargo test --locked -p builtins --features test-support`: 160 passed, 0 failed.
- `cargo test --locked -p builtins-compiler --features test-support`: 58 passed, 0 failed.
- `cargo test --locked -p host-core`: 294 passed, 0 failed.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (298 marked regions in 48 files)`.

**Attempt 1 verdict: FAIL** (head `67fb27aaa`). The fixtures were confirmed: regenerating them gives zero diff. The F5 cases were confirmed too. There were three findings.
- **MAJOR:** the per-channel lane carry has no test. F8, F14, G3 and G5 survive (scope item 3). This belongs here, not in #2355, because #2280 rewrote exactly `export_lane` and `import_lane`, and #2355's scope covers no carry.
- **MINOR:** the mute-during-fade-in case goes through the same `Unmuting -> Muted` transition as the unmute case, so it kills no mutant on its own.
- **MINOR:** #2280's spec was still in `.github/ISSUE_SPECS/`.

### Attempt 2

`origin/main` at `8e76a1ae3` was merged in with a normal merge, which had no conflicts. Product code is unchanged; only tests and specs changed.

**Carry test.** Two rows were added inside `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders`, with no sibling test. Each row starts from a bank prepared unmuted at 0 dB.
- Row (a): `set_mute(Right, true, 0)`, with the left channel open.
- Row (b): `set_mute(Both, true, 0)`, then `set_mute(Right, false, 4 * FRAMES)`.

Each row is exported from lane 0 and imported into lane 2 of another bank. The test then asserts, per channel, that `is_muted` and `gain_held` equal the source's. Next it applies the same timed `SetGain { gain: 0.5 }` to both lanes and compares two blocks of `run_fader` words.

Test value: an export or import that takes one channel's phase or countdown for the other's un-mutes a muted right channel across a plan swap, or drops a right channel's unmute hold, so a fader move then steps the ramp. No earlier test carries a lane whose channels differ.

**Fade-in case.** The mute-during-fade-in case stays. It goes through the same `Unmuting -> Muted` transition as the mute-during-unmute case, and it kills no mutant on its own: in the review's run, F5, G1, G8 and G9 are each also red in the unmute test. It is kept as the arm path's own statement of the contract.

**Mutants**, each against `-p builtins --features test-support --lib`, using the reviewer's table (`i2354/muts.py`). All are red.
- F8 and F14 fail on `right muted: channel 1's mute carries`.
- G3 and G5 fail on `right mid-unmute: channel 1's hold carries`.
- All four fail in `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders`.
- F5 fails both mute tests.
- The 18 table mutants all fail: A, C, D, M2, N10, N17, N21, N24, N26, P4, X12, X4, X9, Y14, Y2, Y7, Y8, Y9. N21, P4 and X9 now also fail the carry test.
- F7 is still #2355's.

**No bit moved.** `gr1_bits` on the merged tree gives simd4 `c50ab7ba3c148cb5`, simd8 `c632900598d806ce` and scalar `1d4ee1dfcd9290db`.

**Gates.** Each crate was run on its own.
- `cargo test --locked -p builtins --features test-support`: 160 passed.
- `cargo test --locked -p builtins-compiler --features test-support`: 58 passed.
- `cargo test --locked -p graph-compiler`: 206 passed.
- `cargo test --locked -p host-core`: 294 passed.
- clippy `-D warnings` (builtins, all targets and features), `cargo fmt --check` and rustdoc `-D warnings`: clean.
- Workspace policy: ok.
- Realtime policy: ok (298 marked regions in 48 files).

**Spec hygiene.** #2280's spec is deleted. It was never on `main`, and its text is kept at `f82131236`.

**Attempt 2 verdict: PASS** (head `448a9df74`). Only minor findings remained, and they are folded in attempt 3 below.

### Attempt 3 (fold)

- **One test per claim.** I deleted the mute-during-fade-in rows from `an_arm_holds_the_lane_until_its_fire_across_a_carry_and_a_reset` and removed their clause from that test's doc comment.
  - They kill no mutant on their own. F5, P4 and the G-series are red in `an_unmute_holds_the_gain_only_while_its_countdown_runs` and in the carry test.
  - The `Unmuting -> Muted` claim is now tested once, in the unmute test.
- **Merge.** `origin/main` had no new commits since `8e76a1ae3`, so the merge was a no-op.
- **Gates.**
  - `cargo test --locked -p builtins --features test-support`: 160 passed, 0 failed.
  - clippy `-D warnings` (builtins, all targets and features): clean.
  - `cargo fmt --check`: clean.
  - Workspace policy: ok.

## Follow-ups

These are recorded, not fixed here.
- **H7 (equivalent mutant):** `import_lane` writes the `ramp[channel].remaining` f32 word, but nothing ever reads it, because `process_plane` rebuilds it from the authoritative `u32` countdown. A later cleanup could drop the write.
- **H13 (lane-index gap):** a mutant whose export takes `fader_gain` from lane 0 survives the builtins suite, because the carry tests always export from lane 0. This gap predates this issue.
