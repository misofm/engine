PASS

# #1411 attempt 1 -- adversarial verdict

Commit under review: `4e41b6296` (branch `codex/d15-stream-g`), parent `8deee2c5e` (#1409 attempt 1).
I reviewed `git diff 8deee2c5e 4e41b6296` against the spec at `4e41b6296` (D0-D5, the gates, the
root correction on the gate-3 delay row, the Attempt record), the #1409 attempt-1 verdict, AGENTS.md,
decision 15 and the no-shortcuts principle. All builds and mutations ran in private `git archive`
exports under `/tmp/claude-1002/v1411/` (`tree` = the commit; `mut` and `mut3` = mutation copies),
each with its own fresh `CARGO_TARGET_DIR`. I did not build, edit or check out anything in
`/home/bl/misofm/wt-d15-g`.

The change is correct and complete. All eight restore sites now hold a ramp's `current`, moving or
settled, to the strict domain or designed range, with no rounding budget. Each site's old rule turns
only its new gate-1 test red. All six #1301 probes are red on their #1409 render-site mutants, per
pull request and full, with the counts the spec records, and green after revert. No render code
changed, and no pinned digest or memset count moved. Every gate is green. The findings are one
MINOR (stale probe docs outside this slice's paths) and three NITs.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **The six probe caller tests' docs now name the product as the defect.**
   - Lines: `crates/compressor/tests/randomized.rs:150-151`, `crates/delay/tests/randomized.rs:21-22`,
     `crates/gate-expander/tests/randomized.rs:29-30`,
     `crates/multiband-compressor/tests/randomized.rs:73-74`,
     `crates/transient-shaper/tests/randomized.rs:29-30` and
     `crates/true-peak-limiter/tests/randomized.rs:32-33`.
   - Each doc says that the ramp's "iterated `current + step` has rounded past the edge", and that
     the test is "Red on a restore that holds a moving ramp's `current` ... to the strict domain".
   - After this slice that strict restore is the product, and the probe is green on it. The probe is
     now red only on a render site that loses #1409's clamp (gate 3). If the probe goes red later,
     its doc points the reader at the wrong fix: re-adding the slack.
   - The probe's own doc has the same problem: `crates/conformance/src/randomized.rs:2657-2658`
     ("a ramp that ends on a domain edge can round a few ulps past the edge on its way").
   - The spec makes these files non-goals ("The probes and `crates/conformance/src/randomized.rs`
     (#1301), their caller tests"), so the implementer was right not to edit them. This is a spec
     gap, not an attempt defect.
   - Root: authorize a doc-only fix in this same pull request (for example, in #1409's tests-only
     attempt 2). The fix states the gate-3 render-side catch. Or file a follow-up.

### NIT

1. **Authorized paths omit `crates/compressor/tests/MUTATIONS.md`.** The Attempt record says root
   authorized the `payload` row edit after the first hand-back, but the Authorized paths list does
   not name the file. Add it to the list.
2. **The compressor's gate-3 row reads as if the probe reaches site 2.** I ran each site alone:
   - site 4 alone gives 56 and 56 refusals;
   - site 2 alone gives 0 and 0 (green).

   The probe takes the `Detector::Main` path, which advances the parameter ramps through
   `RampVec::advance_where` (`crates/compressor/src/kernel.rs:964`). Site 2 reaches the compressor
   only through `frames_loop::<_, true>` -> `Channel::advance_ramps` (`kernel.rs:198-229`, `:869`),
   which the probe never takes. The row is red as defined (both sites together). Site 2 in the
   compressor is guarded by #1409's gate-2 "compressor connected" move, not by this probe. A short
   note in the row would prevent a later misreading.
3. **One test doc claims more than its own rows test.** The doc of
   `a_coefficient_below_zero_or_above_its_design_is_refused` (`crates/compressor/tests/payload.rs:393-397`)
   says that a coefficient is held to `(0, 1]` "as a current, moving or settled". Its rows test only
   targets and settled currents. The moving case is in the new test.

## The eight points in the brief

### 1. Restore validation sites: correct and complete

| Site | At `4e41b6296` | Verified |
| --- | --- | --- |
| Compressor parameters, `validate_channel` (`compressor/src/state.rs:116-128`) | `parameter_state_valid` (domain and `-0.0`) on `current` and `target`, whatever `remaining`; `ramp_path_inside(ramp, (min, max), 64)` | yes |
| Compressor rate coefficients (`:136-145`) | `designed` (`0 < c <= 1`) on `target` and `current`, whatever `remaining`; path bounds `(0.0, 1.0)` | yes |
| Gate, `parse_lane` (`gate-expander/src/lib.rs:772-803`) | `resting &&` dropped; slack gone; `ramp_path_inside` with the strict domain; `-0.0` clauses kept | yes |
| Multiband, `stage_side` (`multiband-compressor/src/lib.rs:1438-1448`) | `parameter_state_valid(current)` whatever `remaining`; strict `ramp_path_inside`; settled `current == target` bits clause kept | yes |
| Delay, `read_carried_ramp` (`delay/src/lib.rs:1661-1675`) | `parameter_value_valid(current)` whatever `remaining`; strict `ramp_path_inside`; settled `current == target` kept | yes |
| Transient, `read_lane` (`transient-shaper/src/lib.rs:663-690`) | `value_valid(current)` whatever `remaining`; strict `ramp_path_inside` | yes |
| Limiter, `read_lane` (`true-peak-limiter/src/lib.rs:4071-4081`) | `coefficient_bounds` deleted; `ramp_path_inside(read, (low, high), RAMP_UPDATES)`; target and settled-bits clauses kept | yes |
| Soft clip, `decode_lane_words` (`soft-clip/src/lib.rs:716-724`) | `ramp_current_valid` and `ulp_at` deleted; `converted_value_valid(current)` (finite, not `-0.0`, inside the converted range) | yes |
| `ramp_path_within` (`effect-runtime/src/state_payload.rs`) | deleted; module doc and `ramp_path_inside` doc rewritten | yes |

- **Refuses every out-of-endpoint word.** Every site now checks `current` against the closed
  strict range whatever `remaining` is. `ramp_path_inside`'s `contains` refuses NaN, and so do
  `parameter_value_valid`, `designed` and `converted_value_valid`. On-edge words are admitted:
  every bound is closed, except the compressor coefficient's open `0`, which is the designed range.
- **Admits every word the clamped render produces.** By #1409 D2, every engine ramp word lies
  between the word at the event (or the restore) and an in-domain target. By induction it lies in
  the domain. The evidence is gate 2: all six probes are green per pull request and at
  `MISO_ENGINE_RANDOMIZED_SCALE=1`, and the every-sample walk restores every own snapshot.
- **Signed zero.** No site's `-0.0` rule changed:
  - the compressor, gate and soft clip refuse `-0.0` in every state, as they did before;
  - the multiband, transient and delay admit it as they did before (the multiband and transient
    normalize it).

  A clamp of finite non-`-0.0` operands cannot make `-0.0`. `x + (-x)` gives `+0` in RNE, and min
  and max return one of their operands.
- **Accepted set is now closed under render.** An accepted ramp has `current` and `target` in
  range. The clamp keeps every later word between them, and the snap writes the `+0.0` step. So
  every later snapshot is accepted too. That is why the old soft-clip paragraph ("not closed under
  render") was correctly deleted.
- **Missed slack sites: none.** I searched every crate's `src` for `slack`, `f32::EPSILON`, `ulp`,
  `ramp_path_within`, `coefficient_bounds`, `ramp_current_valid`, "rounding budget/margin",
  `remaining != 0 ||`, `remaining == 0 &&` and `resting &&`. No ramp-range slack is left.
  - `parametric-eq`'s `RAMP_PATH_NORM_TOLERANCE` (`lib.rs:2846`) is an SVF stability-norm
    tolerance on the EQ's unclamped coefficient ramp (#1407/#1409 non-goal). It is not an endpoint
    budget.
  - No fuzz target restores effect state.

### 2. Soft clip (#1409 verifier MINOR 2): covered

- The line clause is gone with `ramp_current_valid`. The replacement keeps the old function's first
  branch: an in-range current is admitted whatever its line. It refuses every out-of-range current,
  on its line or off it. So it refuses a superset of what the line clause refused, and the clause's
  sign and slope have nothing left to guard.
- The new test writes each word one ulp outside each edge, exactly on its own line (`remaining 1`,
  `step = edge - outside`, which is an exact Sterbenz subtraction). The old allowance admitted that
  case.
- I ran two mutants:
  - the old function restored: the new test is red at `ramp 0: current 6.3095726e-2 (0x3d813855)`;
  - the allowance without its line check (any finite in-flight current): the new test and
    `a_restore_rejects_..._every_invalid_word` (row `bad(0, 1)`) are red.
- With the existing at-rest top+1ulp row and the in-flight `-0.0` row, nothing is missing.

### 3. Gate-1 tests: each refuses, keeps the snapshot and admits on-edge

All seven tests:

- render a short block that moves every ramped word on both channels;
- assert that each word is in flight;
- write `current` one ulp outside each edge;
- assert `effect.state.parameter` and an unchanged scalar snapshot (and bank-track snapshot where
  the effect banks);
- restore the on-edge payload, then the own snapshot.

Mutants (each applied alone in `mut`, the owning crate's whole `--all-targets` suite plus
`conformance` run, then reverted; the tree was confirmed pristine after the run):

| Mutant | Red | Only the new test? |
| --- | --- | --- |
| compressor parameters: moving `current` exempt, 64-ulp path | `ramp 0: current -8.000001e1 (0xc2a00001)` | yes |
| compressor coefficients: settled-only `designed`, path `[0, 1 + 64 ulps]` | `ramp 7: current 0e0` | yes |
| gate: `resting &&`, 64-ulp path | `ramp 0: current -8.000001e1` | yes |
| multiband: settled-only domain, 64-ulp path | `ramp 0: current -8.000001e1` | yes |
| delay: `remaining != 0 \|\|`, 64-ulp path | `section 0 word 1: current -1e-45 (0x80000001)` | yes |
| transient: `remaining != 0 \|\|`, 64-ulp path | `ramp 0: current -1.0000001e0` | yes |
| transient: slack on the top edge only / on the bottom edge only | `1.0000001e0` / `-1.0000001e0` | yes / yes |
| limiter: `coefficient_bounds` restored | `word 7: current 5.623413e-2 (0x3d6655c2)` | yes |
| soft clip: `ramp_current_valid` restored | `ramp 0: current 6.3095726e-2` | yes |
| soft clip: allowance without its line check | new test + `a_restore_rejects_...` | no (expected) |
| compressor: a moving coefficient on its `1.0` edge refused | on-edge half red (`ramp 7: current 1e0`) | yes |
| limiter: a moving coefficient on either edge refused | on-edge half red; also `the_effects_own_edge_ramp_snapshots_restore` and `the_bank_renders_..._under_random_state` | no (the engine does hold edge words in flight) |
| bank hook accepts without validating (gate, compressor, transient, limiter) | each red at its bank `expect_err` / bank assertion | yes |

- The bank-hook mutants prove that the `Option`-guarded bank halves run on this host. The
  multiband and soft clip banks are unconditional.
- The tests read the left section only. Both sections go through the same reader, so this is
  enough.

### 4. Gate 3 (merge requirement): reproduced, all six red

The mutants were applied in `mut3`, one at a time. `refused` counts lines `its own snapshot is
refused`.

| Probe | Mutant | Per PR | Full (`=1`) | Lists | After revert |
| --- | --- | --- | --- | --- | --- |
| compressor | site 4 + site 2 | red, 56 | red, 56 | identical | green |
| gate | site 5 | red, 32 | red, 32 | identical | green |
| multiband | site 6 | red, 80 | red, 80 | identical | green |
| limiter | site 9 | red, 4 | red, 4 | differ (first sample 62/62/56/60 per PR, 54 full; same four `ceiling` ramps) | green |
| transient | site 2 | red, 24 | red, 24 | identical | green |
| delay | site 2 (`next_value` + `advance_block` first word) | red, 24 (feedback 8, mix 8, cross 8) | red, 24 | identical | green |
| delay | site 7 `ramp_word` alone | green, 0 | green, 0 | identical | green |
| compressor | site 4 alone | red, 56 | red, 56 | identical | green |
| compressor | site 2 alone | green, 0 | green, 0 | identical | green |

Every count and the limiter's sample positions match the Attempt record.

**The root's delay correction is sound.**

- The snapshot writes `DelayLane::ramps` (`delay/src/lib.rs:1549-1550`) and the cross `LinearRamp`
  (`:1364`). These are advanced only by `LinearRamp::advance_block` -> `next_value` in `chunk_of`
  (`:1026-1028`) and in the cross segment (`:888`), which is site 2.
- `delay_chunk`'s `ramp_word` (site 7, `:1161-1167`) iterates locals seeded from the segment's
  `start` and writes back only the damping filter states. So its words are render-local and never
  reach the payload.
- Site 7 stays gated by #1409's delay `tests/ramp_endpoint.rs`. That test's reach is #1409's open
  MAJOR, for its attempt 2.

### 5. Bit identity: nothing moved

- **No render code changed.** Every non-test source hunk is in a payload reader, in the deleted
  helpers, or in docs. No fixture or digest file changed.
- **Pins.** `g5_native_digests_match_pins` passes (release). `conformance_fixtures -- --check`
  passes. The browser `expected.json` digests agree with the built simd128 module.
- **Restore continuation.** Every existing mid-ramp restore continuation and round-trip test is
  green, debug and per test-debug-b.
- **Memset.** The iOS memset counts are unchanged from #1409's verified table: compressor 970,
  gate 91, multiband 566, limiter 104, transient 268, soft clip 22.

### 6. Docs and comments: the #1409-listed stale text is fixed

- The soft clip `ramp_current_valid` doc was deleted with the function. `converted_value_valid`'s
  and `decode_lane_words`' docs are rewritten.
- These comments now cite #1409 D2 / #1411 D1:
  - compressor `state.rs` (both comments);
  - gate `lib.rs:779-784`;
  - multiband `lib.rs:1431-1435`;
  - transient `lib.rs:657-661`.
- Ledgers:
  - delay MUTATIONS M18 is deleted, and nothing else refers to it;
  - M19 names `ramp_path_inside`;
  - compressor `MUTATIONS.md:83` says `(0, 1]` (issue #1411).
- No added line exceeds 100 columns.
- The probe-side stale text is MINOR 1.

### 7. Gates (export `tree`, fresh target dirs): all pass

| Gate | Result |
| --- | --- |
| `cargo test --locked --all-targets -p effect-runtime -p compressor -p gate-expander -p multiband-compressor -p delay -p transient-shaper -p true-peak-limiter -p soft-clip -p conformance` | pass (104 result lines, 0 failed) |
| `MISO_ENGINE_RANDOMIZED_SCALE=1 cargo test --locked -p (seven effects) --test randomized` | pass (delay 70.0 s) |
| test-debug-b command (DSP crates, explicit features) | pass (160 result lines, 0 failed) |
| test-debug-a command (workspace minus DSP/audit/wasm tools) | pass (122 result lines, 0 failed) |
| `conformance_fixtures -- --check` | pass |
| `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass (`g5_native_digests_match_pins` ok) |
| `bash scripts/run-wasm-gates.sh` | pass (native, simd128, V8 spill gate) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | pass |
| `check-cross-targets.sh` | pass (memset counts unchanged) |
| `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh .`, `check-realtime-policy.sh`, `check-workspace-policy.sh` | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | pass |
| `cargo fmt --all -- --check` | pass |

### 8. GitHub

#1411's body equals the spec at `4e41b6296` byte for byte (26,080 bytes). Its title matches and it
is OPEN.

## Test value (the seven new tests)

Each test is `a_moving_ramp_word_past_its_domain_is_refused`:

- **compressor** (`tests/payload.rs`): red when `validate_channel` exempts a moving parameter
  `current` from the domain, keeps a rounding budget, or holds a moving coefficient only to
  `[0, 1 + 64 ulps]`. Both mutants were verified red, and no other test catches either.
- **gate** (`tests/state.rs`): red when `parse_lane` keeps `resting &&` with the 64-ulp path
  (verified; no other test).
- **multiband** (`tests/product.rs`): red when `stage_side` domain-checks `current` only at rest
  with a budget (verified; no other test).
- **delay** (`src/lib.rs` tests): red when `read_carried_ramp` restores `remaining != 0 ||` and the
  budget (verified on the cross ramp; no other test).
- **transient** (`tests/contract.rs`): red when `read_lane` keeps a budget on either edge (verified
  for both edges and for each one-sided slack; no other test).
- **limiter** (`src/lib.rs` tests): red when `coefficient_bounds` widens a moving coefficient's
  bounds (verified; no other test).
- **soft clip** (`tests/state_roundtrip.rs`): red when `decode_lane_words` admits an in-flight
  `current` outside the converted range, even exactly on its own line (verified; the old allowance
  is caught only here).

Each test also defends two more claims:

- a refused restore leaves the scalar and bank state unchanged;
- an on-edge moving word restores. The edge mutants were verified red.
