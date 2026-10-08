# Pin ramp_toward's operand order, signed zero and NaN in the wasm corpus

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-07 by root order, as the successor named by the attempt 1 verdict on *Replay an
effect ramp whose endpoint clamp acts in the wasm corpus* (#1459, MINOR-3) and by *Give the
multiband compressor an unarmed form for live audio* (#1455, its corrected Test value). Ordered
after #1459 and #1455. Verify every code anchor on the branch before starting.

## Product outcome

The wasm `simd128` build is pinned, by the G5 cross-target corpus, on two behaviors that no wasm
digest covers today:

1. `lane::kernels::ramp_toward`'s operand order, which is part of its contract: an in-range word
   keeps the unclamped law's exact bits, including `+0.0` next to a `-0.0` endpoint and the
   reverse, and a NaN `next` passes through rather than being hidden. A wasm lowering that swaps
   the `min`/`max` operands, or replaces them with a `clamp`, turns a G5 case red under wasm.
2. The multiband compressor's per-segment dispatch (`process_block` -> `run_segment`'s armed and
   unarmed forms, #1455). A wasm difference in which segments arm, or in the unarmed form's bits,
   turns a G5 case red under wasm.

## Context

- **`ramp_toward`** (`crates/lane/src/kernels/builtins.rs`, "Operand order is part of the
  contract"): `L::min(high, L::max(low, next))` with `low = L::min(current, target)` and
  `high = L::max(current, target)`. `Lane::max`/`Lane::min` keep their **second** operand unless
  the first is strictly beyond it. Every launch effect's ramp (#1409) and the builtins ramps call
  it.
- **Why #1459's case does not cover it.** Case 124 (`gate_expander` `dual_mono/clamped_ramp`) has
  only finite nonzero words. The #1459 verifier swapped the operands
  (`L::min(L::max(next, low), high)`) and no G5 digest moved, case 124 included. For finite nonzero
  operands the two orders agree; they differ only when an operand is a signed zero equal in value
  to another, or NaN.
- **NaN and D5.** The G5 corpora exclude NaN payloads because wasm arithmetic may canonicalize them
  (master plan D5; `crates/effect-runtime/src/corpus.rs`, "No NaN"). `pmin`/`pmax` are bit selects,
  but `current + step` that produces a NaN is arithmetic. The NaN arm of this case can therefore pin
  only whether a result word is NaN (as one canonical token), never its payload.
- **The multiband corpus.** `crates/multiband-compressor/src/corpus.rs` `run_case` drives `lr4_step`
  with its own counter and never calls `process_block`; `tools/wasm-gate-corpus` `digest_multiband`
  delegates to it. No browser document runs an unbypassed multiband through a digest. So no wasm
  digest reaches `process_block`'s per-segment dispatch (#1455 record, "Wasm digest coverage of the
  dispatch: none").
- **Pins.** The G5 pins are the one cross-target corpus AGENTS.md allows a digest for; the single
  owner is `wasm-gates`' `g5_native_digests_match_pins`, and each crate's table holds its rows. A
  new row comes from the scalar oracle, never from production output.

## Decisions frozen for this slice

- **D1. The ramp case.** One new G5 case that drives `ramp_toward` at every width over directed
  points where operand order matters: `next` equal in value to an endpoint of the opposite zero
  sign (`+0.0` against `-0.0` and the reverse, as `low` and as `high`), `current` and `target` of
  opposite zero signs, and `next` NaN (from a NaN `step`), plus ordinary in-range and out-of-range
  points. Home: the effect-runtime D1 corpus (`crates/effect-runtime/src/corpus.rs`), whose cases
  are lane-generic functions; if a production effect path reaches these words in a reachable state,
  its corpus may hold the case instead, and the record states which and why.
- **D2. NaN in the digest.** The case's hash maps every NaN result word to one fixed token before
  hashing, so the pin depends on NaN-ness, not on a payload. The module's "No NaN" doc is amended
  to state this one exception and its reason. The `tests/determinism.rs` NaN-free check is amended
  to match, not removed.
- **D3. The multiband case.** One new multiband G5 case that renders through the product
  `process_block` (not `lr4_step` alone) over blocks in which some segments arm and some do not
  (silent and live frames in each channel and crossover stage, in dual-mono and in a linked
  mode), so the per-segment dispatch and both `run_segment` forms run. It is hashed
  lane-major like the existing cases. No existing multiband pin moves.
- **D4.** No product code changes. No rendered bit moves outside the new rows.

## Deliverables

- The ramp case and its pin row (D1, D2), its name in the case list, and the corpus doc.
- The multiband case and its pin row (D3).
- `tools/wasm-gate-corpus`: only what deriving the new case counts needs (doc or count).
- The record: each case's directed points, each pin's oracle source, and the mutation runs.

## Authorized paths

- `crates/effect-runtime/src/corpus.rs` (its `D1_DIGESTS` table is inline) and
  `crates/effect-runtime/tests/determinism.rs` (D1, D2), or the corpus of the effect named under D1.
- `crates/multiband-compressor/src/corpus.rs` and `crates/multiband-compressor/src/corpus_digests.in` (D3).
- `tools/wasm-gate-corpus/src/lib.rs` (counts or docs only).

## Non-goals

- Changing `ramp_toward`, `Lane::min`/`max`, or any effect's render code.
- Pinning NaN payloads.
- A browser (AudioWorklet) digest of the multiband.

## Objective gates

1. **Pins.** `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   passes with the new rows and no existing pin moved; `bash scripts/run-wasm-gates.sh` passes (the
   `simd128` leg replays both new cases with 0 mismatches).
2. **Red, operand order.** `ramp_toward` changed to `L::min(L::max(next, low), high)`: the ramp
   case's digest moves natively at every width (scalar, simd4, simd8). Also red: `L::max(L::min(next,
   high), low)` and a form that hides a NaN `next`. Reverted: green. Record each.
3. **Red, dispatch.** Each of: the per-segment arming test reading one channel only; reading one
   crossover stage only; every segment forced unarmed; every segment forced armed where the unarmed
   form's bits differ: the multiband case's digest moves. Reverted: green. If one of these changes
   no bit by construction (the forms agree on that segment), the record says so with the reason.
4. **No other case moves** under the mutants in gates 2 and 3 that no existing case already
   catches; the record lists which existing cases move.
5. `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`,
   `bash scripts/check-workspace-policy.sh` pass.

## Test value

The ramp case: a wasm `simd128` lowering of `ramp_toward` that swaps the `pmin`/`pmax` operands or
hides a NaN turns it red under wasm; no existing case reaches it (#1459 verdict, measured). The
multiband case: a wasm difference in the per-segment dispatch turns it red under wasm; no wasm
digest reaches `process_block` today.

## Dependencies

- #1459 and #1455 (stream G).

## Attempt record

### Attempt 1 (2026-10-08, implementer)

All code anchors verified on `codex/d15-stream-g3` at `3c1f4ee23` before starting: `ramp_toward`
(`crates/lane/src/kernels/builtins.rs:222-227`), `run_case`/`D1_DIGESTS` in
`crates/effect-runtime/src/corpus.rs`, the multiband corpus that never calls `process_block`, and
`process_block`/`run_segment`/`Side::arms` in `crates/multiband-compressor/src/lib.rs`. No product
code changed (D4).

**The ramp case (D1, D2).** Home: the effect-runtime D1 corpus, case 9 `ramp_toward` (G5 global
case 98), because it is a lane-generic function and no production effect reaches signed-zero
endpoints in a reachable state (every effect admits parameter words through `normalize_zero`).
Points 0..269 are the directed triples `current x step x target` over `current` in
`{+0, -0, 1, -1, 0.25}`, `step` in `{+0, -0, 1, -1, 0.5, -0.5, 2, -2, NaN (0x7fd0_1473)}` and
`target` in `{+0, -0, 1, -1, 0.25, 0.75}`: `next` equal in value to an endpoint of the opposite zero
sign (as `low` and as `high`), `current` and `target` zeros of opposite sign, `next` NaN from a NaN
`step`, and ordinary in-range and out-of-range points. Points 270.. are drawn on a `2^-16` grid
(`current`, `target` in `[-2, 2]`, `step` in `[-1, 1]`), exact on every target. Every NaN result
word is replaced by `NAN_TOKEN = 0x7fc0_0000` (an integer test on the bits) before hashing, so the
pin depends on NaN-ness only. The module's "No NaN" doc now states this one exception and its
reason; `tests/determinism.rs` `the_corpus_is_nan_free` is amended, not removed: every other case
is still NaN-free, `ramp_toward` emits NaN only as the token, and it emits the token at least once.
`every_case_has_a_wide_output_spread` keeps the `POINTS / 4` floor for the case (a `2^-12` grid
first failed it with 15,988 distinct words; the `2^-16` grid passes).

**The multiband case (D3).** Case 6 `process_block/segment_dispatch` (G5 global case 110). Eight
tracks, prepared in groups of `W` as one `Instance<L, W>` from `initial_defaults` and
`expected_prepared_metadata` with the registry's tail-bound entry, rendered through `render` and so
the product `process_block`, 4 blocks of 128 frames at 48 kHz, in `DualMono` then `Maximum`, read
back lane major (`mode, track, channel, frame`): 2 x 8 x 2 x 512 = `POINTS` words, so the crate's
existing `POINTS`-shaped finiteness test covers it unchanged. Arming needs `N_SILENCE` = 4,096 zero
frames, which no 512-frame render reaches, so four channels start from a restored crossover state
through the product's plan-swap door (`Instance::snapshot`, patched filter and silence words,
`Instance::restore`, validated like any payload): 80 Hz crossover, words of order `1e-16` (inside
`[FLUSH_EPS, REST_EPS)`), unequal band makeups so an unzeroed word reaches the output.

| track | channel | counter | held stages | arming segment | defends |
|---|---|---|---|---|---|
| 0 | left | 3,990 | `a` only | block 0, first after restore | stage `b`-only, right-only, all-unarmed |
| 1 | left | 3,900 | `b` only | block 1 | stage `a`-only, right-only, all-unarmed |
| 2 | right | 3,800 | both | block 2's ramping head (armed) | left-only, all-unarmed |
| 3 | right | 3,620 | both | block 3's flat tail (head unarmed) | left-only, all-unarmed |

Automation points on tracks 2 and 3 (their own seeded channels) split blocks 2 and 3 into a 63-frame
ramping head and a flat tail, so all four `run_segment` forms run (ramping x armable) and block 3
advances the counter over its unarmed head with `silence_skip_block`. Every other channel is live
with 23-frame zero gaps. First draft found a width dependence: with the split point on a live track
(5), the W8 bank split track 0's block and its stage `a` fed stage `b` before the arming segment,
so the stage-`b`-only mutant moved scalar and simd4 but not simd8; the points were moved to the
seeded tracks, so each arming segment is the same at every width, and the case was re-derived
before its pin was written.

**Pins.** Both rows come from the scalar oracle `g5_native_digests_match_pins` prints, equal at
scalar, simd4 and simd8 before pinning, and no other case moved: `D1_DIGESTS[9]` =
`ae134091…97f24de6`, `corpus_digests.in` row 7 = `8e2cee2c…a93b14a1`. Inserting the two cases
moves later G5 global indices up (the gate-expander `clamped_ramp` case, 124 at #1459, is now
126); every family's pins are indexed in their own crates, and `wasm-gate-corpus` derives its
counts from `CASE_COUNT` (doc changes only there).

**Gate 2 (red, operand order).** Each mutant applied to `ramp_toward`, then
`cargo test --release -p wasm-gates --test g5_native_corpus g5_native_digests_match_pins`:

| mutant | G5 | other G5 cases moved | existing native tests red |
|---|---|---|---|
| `L::min(L::max(next, low), high)` | case 98 red at scalar, simd4, simd8 | none | `lane` `ramp_toward_holds_the_target_and_keeps_in_range_bits_at_every_width`, `effect-runtime` `negative_zero_remains_armed_and_snaps_back_to_its_sign`, `the_corpus_is_nan_free` (no token) |
| `L::max(L::min(next, high), low)` | case 98 red at all three | none | same three |
| NaN hider `r = min(high, max(low, next)); select(r == r, r, low)` | case 98 red at all three | none | `ramp_toward_holds_…`, `the_corpus_is_nan_free` |

Reverted: green (7 of 7 G5 tests).

**Gate 3 (red, dispatch).** Each mutant applied to `process_block`/`Side::arms`, same command:

| mutant | G5 | other G5 cases moved | existing native tests red (`-p multiband-compressor`) |
|---|---|---|---|
| arming reads the left channel only | case 110 red at all three | none | `either_channel_and_either_stage_arm_the_crossover` |
| arming reads the right channel only | case 110 red at all three | none | same |
| `svf_state_held([a])` (stage `a` only) | case 110 red at all three | none | same |
| `svf_state_held([b])` (stage `b` only) | case 110 red at all three | none | same |
| every segment unarmed (`armable = BYPASS`) | case 110 red at all three | none | same, and `the_crossover_joint_flush_arms_after_its_inputs_silence` |
| every segment armed (`armable = true`) | green, by construction | none | — |

Every segment forced armed changes no bit, by construction: a segment runs unarmed only when
`Side::arms` is false on both sides, and there `silence_armable_holding` proves every lane's rest
threshold is `+0.0` or the lane holds no word for the joint term to zero, so `flush_pair(n1, n2,
rest)` equals two per-word flushes, and `silence_skip_block` leaves each counter where the frame
loop's `silence_step` would. Reverted: green.

**Gate 4.** No existing G5 case moved under any mutant above. Each mutant is also caught natively
by an existing crate test (listed), as expected: those tests do not run in the wasm guest. The new
cases are the only G5 digests that do, so a `simd128` lowering of `Lane::min`/`max` that swaps
operands or hides a NaN, or a wasm difference in the per-segment dispatch, turns them red under
wasm and nothing else in the wasm leg.

**Test value.**
- `runtime/ramp_toward`: a wasm `simd128` lowering (or a `ramp_toward` rewrite) that swaps the
  `pmin`/`pmax` operands, nests them the other way, or hides a NaN `next` turns it red under wasm;
  no other wasm digest reaches those points (#1459 verdict; measured again above).
- `multiband/process_block/segment_dispatch`: a wasm difference in which segments arm (either
  channel, either stage) or in the unarmed form's bits turns it red under wasm; no other wasm digest
  reaches `process_block`.
- `the_corpus_is_nan_free` (amended): a case that hashes a NaN payload instead of the token, or a
  `ramp_toward` case that no longer reaches its NaN arm, turns it red. The NaN `step` carries the
  payload `0x7fd0_1473`, not the token's, so a native `current + step` keeps a payload the token
  mapping must replace: dropping the mapping (`if false && …`) turned it red natively ("ramp_toward
  point 48 is a NaN payload"), reverted green; the pin is unchanged by the payload choice, since
  only NaN-ness is hashed. It is also red under all three gate-2 mutants, which hide the NaN.

**Gates (final tree).**

| gate | command | result |
|---|---|---|
| 1 (pins) | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass, 128 tests, 0 failed; `g5_native_digests_match_pins` green with no existing pin moved |
| 1 (wasm) | `bash scripts/run-wasm-gates.sh` | pass: native leg 146 cases / 370 comparisons, wasm `simd128` leg 146 cases / 258 comparisons, 0 mismatches each (both new cases replayed at scalar and simd4); V8 spill gate ok |
| crates | `cargo test --locked --release -p effect-runtime -p multiband-compressor` | pass, 149 tests, 0 failed |
| 5 | `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `bash scripts/check-workspace-policy.sh` | all pass |
| extra | `bash scripts/check-realtime-policy.sh` | pass |
| worklet | shipped AudioWorklet module (`build-web-audioworklet.sh --module-only`) at the parent `3c1f4ee23` (export) and as built by `run-wasm-gates.sh` on this tree | identical: `70fd8a43…29deddcc` (3,115,873 B), named twin `1d350cfa…8fdbc032`; the corpus modules do not reach the browser artifact, so the rest of the worklet chain has nothing to check |

**Open items.** `tools/wasm-gates/tests/g5_native_corpus.rs`'s module doc still says "the corpus
carries no NaN into a digest"; it is outside this slice's authorized paths, and its
`g5_lane_corpus_is_finite` test covers lane cases only, which stay NaN-free, so nothing there is
wrong in behaviour, only in that sentence's reach.
