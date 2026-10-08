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
