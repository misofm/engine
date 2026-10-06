# Remove the 64-ulp restore slack once every effect ramp is clamped

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-05 as the follow-up that *Keep every effect parameter ramp inside its endpoints*
(#1409) flags to root in its Non-goals. Code anchors verified on `codex/d15-stream-g` at
`1a2a6bc4b`, before #1301 and #1409 land; #1409 moves the lines of
`crates/effect-runtime/src/state_payload.rs` and of the test rows it deletes, so the implementer
re-reads each anchor on the merged tree first.

## Product outcome

An effect's state restore accepts a ramp word only if it is a value the engine can render. Today
every effect's payload reader admits a moving ramp's `current` up to 64 ulps (soft clip: 64 ulps of
`2 * high` off the ramp's own line) past its parameter's domain or designed range, because the
unclamped D11 law let the effect's own ramps round past their targets. After #1409 no engine ramp
word leaves `[min(start, target), max(start, target)]`, and both endpoints are in the domain, so the
slack admits only words the engine never holds. After this slice every reader holds every ramp word,
moving or settled, to its strict domain, and every snapshot the engine makes still restores and
continues bit for bit.

## Context

- **Why the slack is now headroom.** #1409 D1-D2: every effect ramp word is
  `ramp_toward(current, step, target)`, inside `[min(current, target), max(current, target)]`, so by
  induction inside `[min(w0, T), max(w0, T)]`, where `w0` (the value at the event or the restore) and
  `T` (a target the reader holds to the domain) are in the domain. A carried word is therefore inside
  the strict domain on every frame. The engine persists no effect state (owner ruling R6b), so the
  only payloads a reader sees from the engine come from the same binary, and every one was made under
  the clamped law.
- **The shared helper.** `ramp_path_within` (`crates/effect-runtime/src/state_payload.rs:271-291`)
  widens `[low, high]` by `slack` and calls `ramp_path_inside` (`:293-318`; #1409 D6 deletes its
  walk). The module doc names it (`:37`). Its doc (`:280-282`) states the slack as a rounding budget
  for an effect's own ramps.
- **The slack sites (every effect restore validator):**
  1. Compressor parameters, `validate_channel` (`crates/compressor/src/state.rs:105`): `slack`
     `:118`; a moving `current` is held only to `!is_negative_zero` (`:123-127`, comment
     `:119-122`); `ramp_path_within` `:130`.
  2. Compressor rate coefficients (same function): a moving coefficient's path is held to
     `[0, 1 + 64 ulps]` (`:150`, comment `:135-144`), a target and a settled current to `(0, 1]`
     (`:147-149`). Its test `a_coefficient_below_zero_or_above_its_design_is_refused`
     (`crates/compressor/tests/payload.rs:426`, doc `:420-424`) states that budget.
  3. Gate, `parse_lane` (`crates/gate-expander/src/lib.rs:743`): `slack` `:786` (comment
     `:780-784`), `ramp_path_within` `:788-797`, and a moving `current` exempt from the domain
     (`(resting && !parameter_value_valid(spec, current))`, `:801`).
  4. Multiband (`crates/multiband-compressor/src/lib.rs`): `slack` `:1388` (comment `:1383-1386`),
     a settled-only domain check on `current` and `ramp_path_within` `:1395-1397`.
  5. Delay, `read_carried_ramp` (`crates/delay/src/lib.rs:1615`, doc `:1607-1614`): `slack`
     `:1622`, `current_valid = read.remaining != 0 || ...` `:1623`, `ramp_path_within` `:1626`.
  6. Transient shaper, `read_lane` (`crates/transient-shaper/src/lib.rs:662`, doc `:656-661`):
     `slack` `:676`, comment `:677-679`, `current_valid` `:680`, `ramp_path_within` `:683`.
  7. True-peak limiter: `coefficient_bounds` (`crates/true-peak-limiter/src/lib.rs:3952-3964`)
     widens a moving coefficient's path by 64 ulps, used by `read_lane` (`:4021`) at `:4084`
     (comment `:4078-4081`).
  8. Soft clip: `ramp_current_valid` (`crates/soft-clip/src/lib.rs:275-322`, with `ulp_at`
     `:270-273`) admits an in-flight `current` outside the converted range within
     `64 * ulp(2 * high)` of `target - remaining * step`; `decode_lane_words` (`:733`, doc
     `:705-732`, the allowance bullet `:714-715`) calls it at `:745`.
- **Ledger rows that describe the slack.** Delay M18 (`crates/delay/tests/MUTATIONS.md:36`) is the
  mutation "hold a moving `current` strictly to the domain and drop the rounding budget": after this
  slice that is the product. M19 (`:38`) names the `ramp_path_within` clause.
- **The own-snapshot probes.** Six `the_effects_own_edge_ramp_snapshots_restore` tests (compressor,
  delay, gate, multiband, limiter, transient shaper; `tests/randomized.rs` in each), driven by
  `EffectDifferential::assert_edge_ramps_restore` in `crates/conformance/src/randomized.rs` as #1301
  leaves it, restore an effect's own mid-ramp snapshots toward every domain edge. Soft clip's
  `a_restored_near_edge_ramp_continues_bit_for_bit` (`crates/soft-clip/tests/randomized.rs:199`,
  without its crossing count after #1409 D8) does the same with continuation. With the slack gone,
  a render site that loses #1409's clamp makes its effect's own snapshot fail these.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-05).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), filed this follow-up to #1409 in
  stream A (moved to stream G the same day, below): once every effect ramp is clamped (#1409), every effect's restore refuses any ramp word
  outside its strict domain or designed range, with no rounding slack, and every state the engine
  produces still restores bit for bit. Rationale: a validator that admits words the engine never
  holds is a wider contract than the product needs, and the slack's only justification (#1278: the
  unclamped law's own overshoot) is gone. Stream A owns effect payload code and filed it.
  Moved into G so #1409 and #1411 merge together; no window with blind probes on main. Root
  decision (2026-10-05, same delegation): #1409 alone leaves #1301's six probe tests unable to
  catch anything (its clamp keeps every engine snapshot inside the domain, so the 64-ulp slack
  admits every word the probes could see refused), so main must never carry #1409 without this
  slice. #1409 and #1411 land in the same Stream G pull request, #1409's commits first; neither
  merges alone. Stream G owns this slice; the effect payload readers it edits stay stream A's (and
  the effect owners') code, edited here as a named exception.
- **D1. One strict rule.** For every ramped word of every effect: the target, and the `current`
  whether the ramp is moving or settled, lie in the parameter's strict domain (as the reader already
  checks for a target and a settled current: `parameter_state_valid`, `parameter_value_valid`,
  `value_valid`, `converted_value_valid`), or for a designed coefficient in its designed range
  (compressor `(0, 1]`; limiter `[low, high]` of `read_lane`). Every other clause (`remaining`
  bound, finite step, settled `+0.0` step, settled `current == target`, `-0.0`, soft clip's
  `ramp_step_valid`) is unchanged.
- **D2. The helper.** `ramp_path_within` is deleted; each caller calls
  `ramp_path_inside(ramp, (low, high), max_remaining)` with its strict bounds. The module doc and
  `ramp_path_inside`'s doc lose the rounding-budget wording. No new helper.
- **D3. Sites.** Compressor: `current_valid` is the domain check for every ramp, and the rate
  coefficients' bounds are `designed` for the current too. Gate: the `resting &&` qualifier at `:801`
  is dropped. Multiband, delay, transient shaper: `current` is domain-checked whatever `remaining`.
  Limiter: `coefficient_bounds` is deleted and `read_lane` passes `(low, high)`. Soft clip:
  `ramp_current_valid` reduces to `converted_value_valid` (with its `-0.0` and finite rules), and
  `ulp_at`, if then unused, is deleted. Every comment and doc that states the slack or the overshoot
  is rewritten to cite #1409 D2.
- **D4. Nothing renders differently.** The slice changes only what restore accepts; no render
  code, state layout, payload word or digest table changes.
- **D5. The ledgers.** Delay M18 is deleted (its mutation is now the product) and M19's text names
  `ramp_path_inside`. No new ledger row.

## DSP evidence (AGENTS.md)

- **Equations:** a carried word `w` is accepted iff `w` lies in `[min, max]` of its parameter's
  domain (or designed range); with #1409's `w[k+1] = clamp(w[k] + s, min(w[k], T), max(w[k], T))`
  every engine word satisfies it.
- **Coefficient and update rules:** unchanged; no design function changes.
- **Numerical limits:** the slack was 64 ulps of the larger domain edge (soft clip: 64 ulps of
  `2 * high` about the ramp line); after this slice zero.
- **Latency and tail:** unchanged. **Units and smoothing:** unchanged.
- **Denormal/NaN:** unchanged: NaN steps and `-0.0` words stay refused; subnormals stay legal only
  where the domain admits them (soft clip's mix).
- **Fixtures and objective tests:** gates 1-3. **Benchmarks:** none; restore runs in the swap
  block and does less work. **Listening:** none; no rendered bit moves.

## Deliverables

1. D1-D3 at the eight sites and the helper.
2. Gate 1's refusal rows; the doc and comment rewrites of D3; D5's ledger edits.
3. Gates 2-3 run, with their evidence in this spec.

## Authorized paths

All of these are edited in the one Stream G pull request that also carries #1409.

- `crates/effect-runtime/src/state_payload.rs` (`ramp_path_within`, the module doc and
  `ramp_path_inside`'s doc only)
- `crates/compressor/src/state.rs`, `crates/gate-expander/src/lib.rs`,
  `crates/multiband-compressor/src/lib.rs`, `crates/delay/src/lib.rs`,
  `crates/transient-shaper/src/lib.rs`, `crates/true-peak-limiter/src/lib.rs`,
  `crates/soft-clip/src/lib.rs`: the payload readers and helpers named in Context, and in the
  delay and limiter files their payload-refusal unit tests
- Gate 1's rows in `crates/compressor/tests/payload.rs` (and the doc at `:420-424`),
  `crates/gate-expander/tests/state.rs`, `crates/multiband-compressor/tests/product.rs`,
  `crates/transient-shaper/tests/contract.rs`, `crates/soft-clip/tests/state_roundtrip.rs`
- `crates/delay/tests/MUTATIONS.md` (M18, M19 only)
- This spec

`crates/effect-runtime` is stream G's column. The effect payload readers and their refusal rows
(stream A's payload code, and the effect owners'), `crates/delay/tests/MUTATIONS.md` (M18, M19) and
the probe evidence are named exceptions for this slice, recorded in
`docs/handoffs/decision-15-2026-10-05/STREAMS.md`.

## Non-goals

- Any render site, ramp law, window or design function (#1409 owns the clamp).
- The probes and `crates/conformance/src/randomized.rs` (#1301), their caller tests, and the seeded
  differentials.
- Restore rules that are not about a ramp word's range (history, envelopes, lengths, versions).
- Rejected alternatives:
  - Keep the slack as defence in depth: it admits words the engine never holds, so it widens the
    accepted set without protecting any engine state.
  - Check a moving `current` against `[min(start, T), max(start, T)]`: the payload carries no
    `start`, and the strict domain is the tightest bound the reader can state.
  - Keep `ramp_path_within` with a zero slack: a second name for `ramp_path_inside`.

## Hazards

- **Order.** This slice is valid only on top of #1409 (and #1301 before it), and it ships in the
  same pull request as #1409: #1409 alone leaves the six probes blind, and this slice alone (without
  #1409's clamp) turns them red, because the effects' own snapshots would be refused. Neither
  order is allowed on main by itself.
- **Hot files.** The payload readers are also edited by stream A's carry slices (#1279, #1280,
  #1282) and #1409 deletes test rows in the same files; rebase onto whichever landed, keep their
  clauses, and change only the range rules.
- **Signed zero.** #1409 D3 keeps signed zeros of in-range words; a clamp never produces `-0.0`
  from finite in-range operands that are not `-0.0`, and `-0.0` stays refused. If a probe finds an
  engine `-0.0`, stop and report it; do not admit `-0.0`.
- **Realtime.** The readers run in the plan-swap block (`REALTIME_POLICY_BEGIN` regions); the
  change removes arithmetic and adds none.
- Size: one rule at eight sites with its rows; if it cannot close in half a day, split the soft clip
  rule (its own allowance) from the other seven.

## Objective gates

1. **Restore refuses any out-of-endpoint ramp word.** One test per effect,
   `a_moving_ramp_word_past_its_domain_is_refused`, in the file of Authorized paths (delay and
   limiter: their `src/lib.rs` test modules): from the effect's own snapshot with every ramp in
   flight (`remaining > 0`), for every ramped word (every parameter ramp and the compressor's and
   limiter's coefficient ramps) and each domain edge, write the moving `current` one ulp outside
   the edge (soft clip: one ulp outside the converted range, with `target` and `step` chosen so
   `current` is on its own line, which the old allowance admitted); the restore returns
   `effect.state.parameter` and leaves the effect's snapshot unchanged, in the scalar instance and
   (where the effect banks) the bank hook. The same payload with `current` at the edge restores.
   Red on revert of any one site's slack removal.
2. **Restored state stays bit-identical.** Unchanged and green: the six
   `the_effects_own_edge_ramp_snapshots_restore` tests both per pull request and with
   `MISO_ENGINE_RANDOMIZED_SCALE=1` (the every-sample walk), soft clip's
   `a_restored_near_edge_ramp_continues_bit_for_bit`, and every crate's existing mid-ramp restore
   continuation and round-trip test (restored state snapshots back to the payload and renders bit for
   bit). No digest table moves.
3. **Every probe catches again (PR evidence, not committed; required for merge).** On the combined
   #1409 + #1411 tree, each of #1301's six probe tests,
   `the_effects_own_edge_ramp_snapshots_restore` in the crate's `tests/randomized.rs` (driven by
   `EffectDifferential::assert_edge_ramps_restore`), is red again on its mutant. A mutant reverts
   #1409's clamp to the old update `current + step` (or the site's start-plus-steps form) at that
   effect's render sites, the #1409 Context numbering:
   - delay (`crates/delay/tests/randomized.rs`): site 2, `LinearRamp::next_value` and
     `advance_block`'s first-frame word as the delay's `chunk_of` (and its cross ramp) reaches them
     (feedback, mix and the shared cross feedback; the parameters #1301's M18 caught). **Root
     correction (2026-10-05), not a weakening:** this row first named site 7, `delay_chunk`, whose
     words are render-local copies of the ramps for one chunk and never reach the payload; the
     snapshot writes `DelayLane::ramps` and the cross `LinearRamp`, which site 2 advances. With site
     7 reverted the probe stays green (0 and 0, per pull request and full), so it cannot be this
     probe's mutant; site 7 stays gated by #1409's delay `tests/ramp_endpoint.rs` (its gate 2). With
     site 2 reverted the probe is red, 24 and 24 refusals (feedback 8, mix 8, cross feedback 8),
     lists byte-identical (attempt 1 record);
   - compressor (`crates/compressor/tests/randomized.rs`): site 4, `RampVec::advance_where`, and
     site 2 as the compressor's `Channel::advance_ramps` reaches it (`LinearRamp::next_value` and
     `advance_block`'s first-frame word);
   - gate (`crates/gate-expander/tests/randomized.rs`): site 5, `channel_step`'s `RAMPING`
     prologue (threshold, ratio, range, hysteresis; the parameters #1301's gate strict-current
     mutant caught);
   - multiband (`crates/multiband-compressor/tests/randomized.rs`): site 6, `run_segment`;
   - true-peak limiter (`crates/true-peak-limiter/tests/randomized.rs`): site 9,
     `RampLanes::advance` (the limit and release coefficients; the ceiling ramp #1301's 4-ulp
     mutant caught);
   - transient shaper (`crates/transient-shaper/tests/randomized.rs`): site 2, `LinearRamp` as the
     shaper's `advance` reaches it (attack amount, sustain amount, mix).

   For each, run the test per pull request (no variable set) and full
   (`MISO_ENGINE_RANDOMIZED_SCALE=1`): both are red with at least one "its own snapshot is refused"
   line; record each count, whether the two violation lists are byte-identical, and green after
   revert, in this spec. #1301's three restore-side mutants (delay M18, gate strict current,
   limiter 4-ulp budget) are this slice's product and are no longer mutants; this render-side
   table replaces them as the probes' recorded catches. If any of the six probes stays green on its
   mutant, the pull request does not merge: stop and report to root with the evidence; do not
   weaken the probe, the mutant or this gate.
4. Commands:
   - `cargo test --locked --all-targets -p effect-runtime -p compressor -p gate-expander -p multiband-compressor -p delay -p transient-shaper -p true-peak-limiter -p soft-clip -p conformance`
   - `MISO_ENGINE_RANDOMIZED_SCALE=1 cargo test --locked -p compressor -p gate-expander -p multiband-compressor -p delay -p transient-shaper -p true-peak-limiter -p soft-clip --test randomized`
   - the `test-debug-a` and `test-debug-b` workspace commands from `.github/workflows/qualification.yml`
   - `bash scripts/check-effect-runtime-policy.sh`, `bash scripts/test-effect-runtime-policy.sh .`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a reader that keeps (or later re-adds) a rounding slack, or exempts a moving `current`
  from the domain, admits a word one ulp outside its domain; no test refuses such a word today, and
  the existing refusal rows test only targets, settled currents, steps and lengths.

## Dependencies

- *Keep every effect parameter ramp inside its endpoints* (#1409, stream G), whose clamp makes every
  engine ramp word lie inside its domain; it ships in the same Stream G pull request as this slice,
  its commits first. #1409 in turn lands after #1301 (stream J), whose probe gates 2 and 3 run.

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**Change (D1-D3, D5).** Restore only; no render code, state layout, payload word or digest table
changed (D4).

- `crates/effect-runtime/src/state_payload.rs`: `ramp_path_within` deleted; the module doc names
  `ramp_path_inside`, whose doc now carries the `remaining`/step rules and states that `[low, high]`
  is the strict domain or designed range with no rounding budget (#1409 D2, #1411 D1).
- Compressor `validate_channel`: every parameter `current` is held by `parameter_state_valid`
  (domain and `-0.0`) whether or not the ramp moves, and `ramp_path_inside` takes the strict
  domain; the rate coefficients hold `current` to `(0, 1]` whatever `remaining`, path bounds
  `(0.0, 1.0)`.
- Gate `parse_lane`: the `resting &&` qualifier and the slack are gone (`ramp_path_inside`).
- Multiband, delay `read_carried_ramp`, transient shaper `read_lane`: `current` is domain-checked
  whatever `remaining`; `ramp_path_inside` with the strict domain.
- Limiter: `coefficient_bounds` deleted; `read_lane` passes `(low, high)`.
- Soft clip: `ramp_current_valid` and `ulp_at` deleted; `decode_lane_words` calls
  `converted_value_valid` for `current`, as for the target. `converted_value_valid`'s and
  `decode_lane_words`' docs rewritten (the "not closed under render" paragraph went with the
  function: under the clamped law a restored in-range ramp stays in range).
- Every comment and doc that stated the slack or the overshoot cites #1409 D2 / #1411 D1,
  including the limiter's restore-corruption comment, the delay's
  `a_carried_ramp_is_refused_unless_its_whole_path_is_valid` doc (now `ramp_path_inside`), the
  compressor payload test doc of `a_coefficient_below_zero_or_above_its_design_is_refused`, three
  comments of soft clip's `a_restore_rejects_a_stale_version_a_wrong_length_and_every_invalid_word`,
  and `crates/compressor/tests/MUTATIONS.md`'s `payload` row (authorized by root after attempt 1's
  first hand-back; it said `[0, 1 + 64 ulps]`).
- D5: delay M18 deleted; M19 names `ramp_path_inside`. No new ledger row.

**Gate 1** (`a_moving_ramp_word_past_its_domain_is_refused`, seven tests: compressor
`tests/payload.rs`, gate `tests/state.rs`, multiband `tests/product.rs`, transient
`tests/contract.rs`, soft clip `tests/state_roundtrip.rs`, delay and limiter `src/lib.rs` test
modules). Each automates every ramped parameter on both channels at sample 0 (shared cross feedback
on the delay) and renders 8 frames, asserts every ramp word is in flight, then for each ramp of the
left section (and the delay's common cross ramp) and each edge writes `current` one ulp outside:
`effect.state.parameter`, the scalar snapshot unchanged, and for the banking effects the same on a
native-width bank track that first restored the scalar snapshot; `current` on the edge restores in
both. Ramps covered: compressor 7 parameters and both rate coefficients (designed `(0, 1]`: the
first word outside below is `0.0`, the edge word `f32::from_bits(1)`); gate 4; multiband 10;
transient 3; delay feedback, damping coefficient `[0, damping_coefficient_max(48 kHz)]`, mix, cross
feedback (no bank: the delay renders per node); limiter limit and release coefficients over
`read_lane`'s bounds; soft clip drive, output (converted gains `db_to_gain_f32` of the dB edges) and
mix, each rewritten as a one-sample ramp onto the edge (`target` the edge, `step` the exact one-ulp
difference, `remaining` 1) so `current = target - remaining * step` lies exactly on its line, which
the old allowance admitted.

Mutation (each site's old rule restored alone, its test run, reverted; scratch script):

| Site reverted | Red |
| --- | --- |
| compressor parameters (moving `current` exempt, 64-ulp path) | `ramp 0: current -8.000001e1 (0xc2a00001)` restores |
| compressor coefficients (`[0, 1 + 64 ulps]`, settled-only design check) | `ramp 7: current 0e0` restores |
| gate (`resting &&`, 64-ulp path) | `ramp 0: current -8.000001e1` restores |
| multiband (settled-only domain, 64-ulp path) | `ramp 0: current -8.000001e1` restores |
| delay (`remaining != 0 ||`, 64-ulp path) | `section 0 word 1: current -1e-45 (0x80000001)` (cross feedback) |
| transient (`remaining != 0 ||`, 64-ulp path) | `ramp 0: current -1.0000001e0` restores |
| limiter (`coefficient_bounds`) | `word 7: current 5.623413e-2 (0x3d6655c2)` |
| soft clip (`ramp_current_valid` with `64 * ulp(2 * high)`) | `ramp 0: current 6.3095726e-2 (0x3d813855)` |

All green after revert. Test value: a reader that keeps or re-adds a rounding budget, or exempts a
moving `current` from the domain, admits a word one ulp outside it; no earlier row wrote a moving
`current` outside the domain.

**Deleted.** `ramp_path_within`, `coefficient_bounds`, `ramp_current_valid`, `ulp_at`; delay M18.
No test was deleted.

**Gate 2.** Green: the six `the_effects_own_edge_ramp_snapshots_restore` per pull request and with
`MISO_ENGINE_RANDOMIZED_SCALE=1`, soft clip's `a_restored_near_edge_ramp_continues_bit_for_bit`
(both runs), and every existing mid-ramp restore continuation and round-trip test (in the gate 4
commands, debug and release). No digest table moved; no bit moved.

**Gate 3** (PR evidence; mutants applied by a scratch script, one at a time, on the combined tree;
"refused" = lines `its own snapshot is refused`):

| Probe | Mutant (#1409 site reverted to `current + step`) | Per PR | Full | Lists identical | After revert |
| --- | --- | --- | --- | --- | --- |
| compressor | site 4 `RampVec::advance_where` and site 2 (`next_value`, `advance_block` first word) | red, 56 | red, 56 | yes | green |
| gate | site 5 `RAMPING` prologue | red, 32 | red, 32 | yes | green |
| multiband | site 6 `run_segment` | red, 80 | red, 80 | yes | green |
| limiter | site 9 `RampLanes::advance` | red, 4 | red, 4 | no (see below) | green |
| transient | site 2 | red, 24 | red, 24 | yes | green |
| delay | site 2, through `chunk_of` (root correction above) | red, 24 | red, 24 | yes | green |

Per parameter: compressor threshold, ratio, knee, attack, release, makeup, mix 8 each (two edges at
four rates); gate threshold, ratio, range, hysteresis 8 each; multiband the ten band parameters 8
each; transient attack amount, sustain amount, mix 8 each; delay feedback, mix, cross feedback 8
each; limiter `ceiling` 4 (toward -24 dB from `0xc1bfffdf`, one per rate). The limiter's two lists
name the same four ramps and differ only in the first refused sample (per pull request 62, 62, 56,
60; full 54 at every rate): the probe stops each ramp at its first refusal, and the per-PR
position set does not hold sample 54. Site 7 (`ramp_word` back to `value + step`) was also run: 0 and 0, green; its words are
render-local and never reach the payload (the root correction in gate 3).

**Gates.**
- `cargo test --locked --all-targets` on the gate 4 package list: pass, debug and `--release`.
- `MISO_ENGINE_RANDOMIZED_SCALE=1 cargo test --locked ... --test randomized` (seven crates): pass.
- `test-debug-a` and `test-debug-b` workspace commands: pass.
- `conformance_fixtures -- --check`: pass.
- `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh .`, `check-realtime-policy.sh`,
  `check-workspace-policy.sh`: pass.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`:
  pass.
- `bash scripts/run-wasm-gates.sh` (native, simd128, V8 spill gate): pass.
- Worklet chain (the readers are in the browser module): `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`: pass.
- `check-cross-targets.sh`: pass; no ceiling touched.

**Open items.** None for this slice.
