# Lane swap: a limiter lane holding a non-finite delay word refuses its own payload, and swap_bank_lanes then leaves a lane half-swapped

Confirmed by the #2285 review (base `24ea03525`) and re-verified on `9099bdb37`; filed under the
implicit-state-machine tracker #2258. The implicit state is a rule that only a `debug_assert`
checks: "a bank never refuses its own lane's payload" (#1279 D6). `swap_bank_lanes` relies on it,
and the limiter breaks it.

## Mission
A render-time lane swap is all or nothing: it either exchanges both lanes or leaves both as they
were. Every bank can restore every payload its own lane writes.

## Verified defect (base `9099bdb37`)
**Limiter.** `crates/true-peak-limiter/src/lib.rs`:
- The kernel classifies no input word (D7). A non-finite input sample is stored verbatim in the
  lane's 12-tap detector `history` and in its `main_ring`, which holds `B = Fs/100 + 6` slots (486
  at 48 kHz). It reaches the output only `B` frames later.
- The §4.4 boundary check scans the output only (`check_block` at 4695).
- In between, every output word is finite. The detector's
  `select(P > limit, limit / P, 1)` answers a NaN peak with `1` and an infinite one with `0`, so
  the gain words stay finite and in range.
- `snapshot_track_state_payload` succeeds. `restore_track_state_payload` of the same payload is
  refused by `read_lane` (5280): `effect.state.history` at 5352-5354 and `effect.state.ring` at
  5387-5389.
- The reviewer's reproducer: W4 at `Maximum`, 4 loud blocks, then a 512-frame block with a NaN at
  frame `k` of lane 0. For 486 of the 512 positions (`k >= 26`) the block's output is finite and
  lane 0's own payload is refused. The scalar instance shares `read_lane` and has the same hole.
  So has the plan-swap carry of either one.

**Lane swap.** `crates/effect-contract/src/live.rs`, `swap_bank_lanes` (3366-3470). Take
first = that lane and second = a donor:
- Step 2 writes the donor's payload into the first lane.
- Step 3 (restoring the first lane's payload into the second) is refused.
- The undo then restores the first lane from its own payload, and that is refused too.
- A debug build panics at 3465 ("a bank never refuses its own lane's payload").
- A release build returns `Err(LaneSwapRefusal::Restore)` with the first lane changed. The
  rustdoc (3365) promises "both lanes unchanged".
- The rack's chain swap (`rack/src/lib.rs` 5103, 5134) then hits its own "a checked slot swaps its
  lanes" `debug_assert`.

**Other effects (survey).** A scratch survey (never committed) prepared every launch effect at
48 kHz with lattice-drawn parameters, 24 seeds each. Each ran 5 loud blocks and then a block with
NaN, `+inf` or `1e37` at frame 0, 64, 120, 126 or 127 on either channel, then restored its own
payload, as a scalar instance and (where it banks) as a W8 bank. 720 cases per effect.
- Only the limiter refused: NaN and `+inf`, frames 0 and 64 with `effect.state.ring`, frames
  120-127 with `effect.state.history`, scalar and bank alike. The finite `1e37` was never refused.
- No refusals: the parametric EQ, compressor, gate/expander, multiband compressor, soft-clip
  (#1300 already admits its non-finite history), transient shaper, delay, de-esser, dynamic EQ
  and reverb.

**Reachability through a compiled session: unreachable today.**
- Every strip's chain starts with the builtin input section (`input -> polarity/trim/HPF/LPF ->
  console pre-insert -> inserts -> ...`), tracks and submixes alike.
- Its sanitise prologue replaces every word whose magnitude is not below `1e30`, every NaN
  included, with `+0.0` (`lane/src/kernels/builtins.rs` 146-168, `NONFINITE_LIMIT` at 78). It is
  used at every copy of the input-chain frame body, the elided ones included.
- Trim is at most +24 dB, so the section's output is finite. Every effect's own D7 check zeroes
  an output that fails `|x| < 1e30`. `conformance/src/effect.rs` 1577 states the same rule: an
  effect never sanitises its input.
- The limiter has no sidechain (`lib.rs` 373).
- So a limiter in a session only ever sees finite words, and a route sum is re-sanitised at the
  submix's input section.
- The defect is reachable through the effect contract alone: a bank or a scalar instance driven
  directly, as the conformance probes, the reviewer's reproducer and any future host do.
- It is still worth fixing:
  - `swap_bank_lanes` documents all or nothing and does not deliver it.
  - The limiter breaks D6, which the carry and the swap both assume.
  - The input-section guarantee is the only thing that keeps either from being reached.

**Severity.** M by the tracker's legend: one forgotten precondition makes a refusal leave a lane
half-swapped in release.

## Decision / model
1. **Validate everything, then write** (the acked-batch lesson in AGENTS.md):
   - New `PreparedNativeEffectBank::validate_track_state_payload(&self, track, version, &input)`.
     It is the write-free verdict of `restore_track_state_payload`: `Ok` exactly when that restore
     would commit, otherwise its refusal.
   - The verdict reads only the payload, the lane's prepared defaults and the bank's prepared
     configuration, never running state. So it stands until the lane's defaults are exchanged.
   - The default declines with `effect.state.unchecked`. A bank that declares lane defaults must
     implement it, and every launch effect's bank does: the parametric EQ, compressor,
     gate/expander, multiband compressor, true-peak limiter, soft-clip, transient shaper, de-esser
     and dynamic EQ.
   - Each implementation is its restore's existing parse/validate half, split out unchanged. The
     restore now calls it and then commits.
   - `swap_bank_lanes` validates both lanes' own payloads against their own lanes before its first
     write, and refuses with a new `LaneSwapRefusal::Payload` that leaves both lanes as they were.
   - The undo after a refused step 3 restores exactly the payload, lane and defaults that passed
     validation. It is now proven rather than assumed, and its `debug_assert` names that contract.
   - No allocation: the validation runs in the swap's existing `REALTIME_POLICY` region on the same
     scratch, and every implementation sits in its crate's payload region.
2. **The limiter's restore admits every word its kernel holds** in the `history` and `main_ring`.
   This is soft-clip's #1071/#1300 rule: "accept every word the effect itself can hold, refuse the
   rest".
   - A lane restored from such a payload reaches the poisoned frame at the same sample and is
     recovered there by the same §4.4 check, so it continues bit for bit.
   - The rest of `read_lane` is unchanged, and nothing else in render changes.
   - **Why not detect at the block end instead?** A ring scan per block costs `O(B)` words per lane
     and channel (486 at 48 kHz, more than a 128-frame block's output), on every block, for a
     state no session can reach. Detecting at the input instead moves the §4.4 rule itself: it
     would reset a lane `B` frames early and move rendered bits on every non-finite input. §4.4
     is defined as `effect-runtime`'s output test, so that change is cross-cutting. Admitting the
     words costs nothing in render and moves no bit.
   - **A crafted payload** holding a non-finite history word without the same sample in the main
     ring (the kernel never writes one; `12 < B`) gets at most 12 frames of `r = 1`. No gain word
     leaves its range, and every output is still a ring word times a gain in `[0, 1]`. R6b means no
     payload comes from outside the engine.

## Non-goals
- A §4.4 state check (see above). Changing the D7 contract would be its own issue.
- Other effects: the survey found none with this asymmetry.
- The rack's chain-level `debug_assert` (a refusing slot is still a defect; with D6 restored
  nothing reaches it).
- The randomized harness's terminal words stop the scenario (`conformance/src/randomized.rs`
  2175-2191), which is why the existing own-snapshot assertion never met this state. Continuing
  after them is harness work, a possible follow-up.

## Gates
- Red on base, green at head:
  - `effect_contract::live::lane_swap_tests::every_lane_swap_refusal_leaves_both_lanes_unchanged`,
    with two new cases: a lane whose own payload no restore accepts, as first and as second.
  - `true_peak_limiter::swap_tests::a_lane_holding_a_non_finite_delay_word_restores_and_swaps_exactly`.
- New, for the new contract: `effect-compiler/tests/restore_verdict.rs`. For every banking launch
  effect, validation answers exactly what the restore then answers.
- No rendered bit moves: a one-time comparison at base and head with the #2285 harness, as
  evidence.
- `cargo test --locked` for the touched crates and the swap's consumers; clippy `-D warnings`;
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`;
  `bash scripts/check-workspace-policy.sh`; the realtime policy tool.

## Evidence (attempt 1)
**Files.**
- `crates/effect-contract/src/lib.rs`: the trait method and its docs, and `lane_default_bytes`'s
  note.
- `crates/effect-contract/src/live.rs`: `LaneSwapRefusal::Payload`, the pre-write validation in
  `swap_bank_lanes` and its rustdoc, and the test bank's validation with two new refusal cases.
- `crates/true-peak-limiter/src/lib.rs`: `parse_track` split out of `restore_track`; `read_lane`
  admits the history and main ring, with the rule written on it; the bank's validation; one stale
  test comment.
- `crates/true-peak-limiter/src/swap_tests.rs`: the new limiter test.
- The parse/validate half split out of each restore, with the bank's validation:
  `compressor`, `de-esser`, `dynamic-eq`, `gate-expander`, `multiband-compressor`,
  `parametric-eq`, `soft-clip` and `transient-shaper` (`src/lib.rs` each).
- The two rack test banks that declare defaults (`rack/tests/lane_rest.rs`, `lane_swap.rs`).
- New: `crates/effect-compiler/tests/restore_verdict.rs`.

**Red on base** (`9099bdb37` with only the new tests; the contract test's two cases expected
`Restore` there, since `Payload` does not exist yet):
- `every_lane_swap_refusal_leaves_both_lanes_unchanged`.
  - Debug: `panicked at crates/effect-contract/src/live.rs:3465:9: a bank never refuses its own
    lane's payload`.
  - Release: `case 10: first bank`. Lane 1 holds `[120, 121, 122]` (the donor's payload), not its
    own `[3735928559, 12, 13]`.
- `a_lane_holding_a_non_finite_delay_word_restores_and_swaps_exactly`:
  `W4 NaN at frame 26 on channel 0: lane 0 restores its own payload`, `left: Err(StatePayloadError
  { code: "effect.state.ring" })`.
- The reviewer's reproducer at head: "no frame position leaves an unrestorable own payload". On
  base, release: `swap returned Err(Restore); first lane unchanged = false`; debug: the panic.

**Test value.**
- The contract test turns red if a swap writes a lane before it knows both lanes' own payloads
  restore. The undo of a refused step 3 then refuses too, and the first lane keeps the donor's
  payload. No existing case has a payload that is refused wherever it is restored: the old
  refusals are call-counted.
- The limiter test turns red if the restore refuses a non-finite history or main-ring word the
  kernel leaves there. It also turns red if a lane restored or swapped with one does not render,
  report and recover bit for bit as its twin. The randomized harness's own-snapshot assertion
  never meets that state, because its terminal words end the scenario.
- `restore_verdict` turns red if an effect's validation skips one of its restore's checks, refuses
  what the restore takes, or keeps the trait's declining default. It runs all 9 banking launch
  effects. It reaches both verdicts and covers the de-esser and dynamic EQ, which have no other
  bank restore test.

**Mutations** (each applied alone, its test run, then reverted; all red):
- `swap_bank_lanes` without the pre-write validation: the contract test panics at the undo's
  `debug_assert`.
- Validation of the first lane only: `case 11: left: Err(Restore), right: Err(Payload)`.
- The limiter's main-ring finiteness check restored: the limiter test fails at frame 26 with
  `effect.state.ring`.
- The limiter's history finiteness check restored: the limiter test fails at frame 511 with
  `effect.state.history`.
- The limiter's validation without its member check: `restore_verdict` fails with
  `miso.true-peak-limiter: track 8` (`Ok` vs `effect.state.track`).
- The de-esser's validation skipping its parse: `restore_verdict` fails on a wrong layout version
  (`Ok` vs `effect.state.version`).
- The EQ's validation without its padded-lane check: `restore_verdict` fails on a padded lane
  (`Ok` vs `effect.bank.track`).

**No rendered bit moved.**
- No kernel, recovery or render body changed. The changes are payload validation, which runs at
  a block boundary, and the swap's pre-write check.
- One-time comparison, never committed: #2285's harness adapted. It is the random 700-step
  schedule of loud, quiet and signed-zero blocks, collapse transitions, automation, both resets,
  lane resets, link retargets, own and donor restores, in-bank and cross-bank lane swaps and
  whole-bank skips, with one FNV-1a digest per case over every output bit, every state word, the
  rest mask and every restore/swap verdict.
- Run in release, W4 and W8, 48 seeds each, on two arms: without NaN blocks and with them.
- NaN-free arm: all 96 digests are identical at base and head.
- NaN arm: 91 of 96 are identical. The 5 that differ (W4 seeds 4 and 25, W8 seeds 11, 28 and 36)
  are exactly the 5 cases where base refused a restore or a swap (one each). Head refused none.
- The shipped arm equals the never-linking reference arm in every case at both commits.

**Gates (head).**
- `cargo test --locked --all-targets` for `effect-contract`, `compressor`,
  `multiband-compressor`, `gate-expander`, `true-peak-limiter`, `transient-shaper`, `soft-clip`,
  `de-esser`, `dynamic-eq`, `parametric-eq` and `conformance` (features
  `parametric-eq/test-support,lane/test-support`): 133 binaries, 686 tests, 0 failed.
- The swap's consumers: `rack`, `graph`, `graph-compiler`, `effect-compiler` and
  `builtins-compiler` (features `graph/test-support,builtins-compiler/test-support,effect-compiler/test-support`):
  51 binaries, 523 tests, 0 failed.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` on the twelve touched
  crates: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` on the twelve touched crates: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (295 marked regions in
  46 files)`. A first run refused `.map(drop)` in the new validations (a function value on a
  marked path), so they end with `?; Ok(())`.
- After that edit, the touched crates' lib and integration tests were rerun with `rack` and
  `effect-compiler` (`effect-compiler/test-support` added): 139 binaries, 791 tests, 0 failed.
  Clippy was clean again.

**Follow-ups (not this issue).**
- The randomized differential could continue past its terminal words with a restore, so the
  own-snapshot rule meets non-finite state in every effect, not only through this issue's
  targeted tests.
- The scalar delay and reverb restores refuse non-normal ring words (`normal_or_zero`). The survey
  found their kernels never leave one, but nothing ties the two.
