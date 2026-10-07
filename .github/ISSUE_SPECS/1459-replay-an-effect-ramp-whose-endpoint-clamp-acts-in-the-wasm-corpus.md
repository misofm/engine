# Replay an effect ramp whose endpoint clamp acts in the wasm corpus

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling (2) on *Keep every effect parameter
ramp inside its endpoints* (#1409, attempt 1 verdict NIT 7). Code anchors verified on
`codex/d15-stream-g` at `c7f7bbdfb`.

## Product outcome

The browser build is proven to clamp an effect's parameter ramp at its target bit for bit as the
native build does. Today the wasm `simd128` leg reaches #1409's clamp only through `lane`'s own
cases (the `ramp_toward` `pmin`/`pmax` lowering); no G5 corpus case renders an effect move in which
the clamp acts. After this slice at least one effect corpus case does, and its digest is replayed
under wasm.

## Context

- **The clamp.** #1409 routes every effect ramp site through
  `lane::kernels::ramp_toward(current, step, target)` (strict `min`/`max`, keeps an in-range word's
  bits, passes NaN). Gate 2 (`tests/ramp_endpoint.rs` per effect) runs native only; `aarch64-debug`
  runs it at NEON width.
- **The G5 corpus.** `tools/wasm-gate-corpus/src/lib.rs` delegates whole rendered effect blocks to
  each effect's `corpus` module (`transient_shaper`, `delay`, `multiband_compressor`, `soft_clip`,
  `parametric_eq`, `gate_expander`, `builtins`), pins their digests in the effect crate (for example
  `crates/gate-expander/src/gate_digests.in`), and replays them under wasm
  (`scripts/run-wasm-gates.sh`); `wasm-gates`' `g5_native_digests_match_pins` is the native owner of
  every pin (#1048). Some cases already have a D11 ramp in flight (gate-expander, parametric EQ),
  but #1409 moved no pin, so no case's ramp is one the clamp changes.
- **The moves.** #1409's scan found, per ramped word, an in-domain move whose unclamped word passed
  its target (for example the gate threshold at frame 33). Such a move is the case to add.

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator filed this slice as the wasm
  reach of #1409's clamp.
- **D1. One effect, one case.** Add one case to the gate-expander corpus (site 5, the gate
  prologue, a vector clamp site at every lane width): a move from #1409's scan, with lanes ending
  their ramps on different frames, in which the clamp acts on at least one frame per lane.
  Another effect may be chosen only with a reason recorded (for example a site the gate case cannot
  reach).
- **D2. New pin, no moved pin.** The new case adds one digest row; every existing pin stays.

## Deliverables

1. The new case in the chosen effect's `corpus` module, its pin in that crate's `.in` file, the
   case count in `tools/wasm-gate-corpus` updated.
2. The attempt record: the clamp acts in the case (the frames where the unclamped word passes its
   target), and the mutation evidence of gate 2.

## Authorized paths

- `crates/gate-expander/src/corpus.rs`, `crates/gate-expander/src/gate_digests.in` (one new row)
- `tools/wasm-gate-corpus/src/lib.rs` (case count and doc only)
- This spec

## Non-goals

- Cases for the other effects; changing the clamp or any render code; native gate-2 tests.

## Hazards

- Hot files: `crates/gate-expander/src/corpus.rs` and `tools/wasm-gate-corpus/src/lib.rs`
  (other slices add G5 cases to the same list); the later slice rebases.
- A move that is clamped only after the window, or on no lane at the native width, is vacuous:
  gate 1 checks reach.

## Objective gates

1. **Reach.** A check in the case's test (native) shows the unclamped law passes the target on at
   least one frame of the case per lane, so the clamp acts.
2. **Pins.** `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   passes with the new row, and no existing pin moved; `bash scripts/run-wasm-gates.sh` passes
   (simd128 leg replays the new case).
3. **Red.** The gate prologue clamp reverted to `add` (#1409's site-5 mutant) moves the new digest
   (red natively); recorded.

## Test value

A wasm `simd128` lowering of an effect's ramp clamp that differs from the native one (operand order
of `pmin`/`pmax`, a `-0.0` or NaN difference inside a real effect render path rather than inside a
lane kernel) turns the new case red under wasm; no existing case reaches it.

## Dependencies

- #1409 (merged with the stream G batch).

## Attempt record

### Attempt 1 (2026-10-07, implementer)

**Case.** `crates/gate-expander/src/corpus.rs` case 6, `dual_mono/clamped_ramp` (G5 case 124),
pinned as the seventh row of `gate_digests.in` from the scalar oracle
(`fb0c11e0…db147186`); `CASE_COUNT` 6 → 7, and `tools/wasm-gate-corpus` derives its count from it
(doc updated only). No existing pin moved: `g5_native_digests_match_pins` printed case 124 only,
equal at scalar, simd4 and simd8, before the pin was written.

**The move (D1).** Every ramped word (threshold, ratio, range, hysteresis) of every lane and
channel starts `0x21` ulps from its resting value, the offset of #1409's scan moves (threshold
`0xc29fffdf → -80.0`, ratio `0x3f800021 → 1.0`, range `0x42bfffdf → 96.0`, hysteresis
`0x41bfffdf → 24.0`), and ramps back to it with the control plane's own step law
(`LinearRamp::set_target`) over a window of `40 + 3·lane + channel` samples (40..62, each lane and
channel ending on a different frame, all inside the 64-frame ramping block). Reason for applying
the scan's offset to each lane's own resting word rather than replaying the four scan moves
verbatim: the scan's ratio move ends at `1.0`, where the expansion curve is zero and the threshold
and range words cannot reach the output; per-lane resting words keep every word live in the
curve. The input is quiet noise at an exact power of two 3 dB under each lane's re-arm level, so
the gate closes after its hold and the curve sets the gain while the clamp acts.

**Gate 1 (reach).** `corpus::tests::the_clamped_case_passes_every_target_under_the_unclamped_law`
replays the unclamped law per lane, channel and word. All 64 words first pass their target at
frame 34, and every window is 40..62, so the clamp acts on frames 34..window−1 (6 to 28 frames)
of every word of every lane at every width. Test value: a case edit that leaves some lane's move
unclamped turns it red; the digest cannot, as it would be re-pinned with the case. Mutations:
window `66 + 3·lane + channel` → red ("window 66 ends past the ramping block"); window
`34 + lane + channel` → red ("stays inside … over its 34-sample window"); offset `0x10` → red
(step under half an ulp, "stays inside … over its 40-sample window"); reverted → green.

**Gate 3 (red).** #1409's site-5 mutant (`let stepped = ramp.current.add(ramp.step);` in
`kernel.rs` `channel_step`) → `g5_native_digests_match_pins` red on case 124 only, at scalar, simd4
and simd8 (got `99502499…18ea64bb`); every other case stayed green, confirming no existing case
reaches the clamp. Per lane output under the mutant: 15 of 16 lane-channels move (first moved
frame 35..49); right channel of lane 0 does not move its output (its clamp acts on the ramp words,
but the bits round away in the one-pole). Reverted → green.

**Gate 2 (pins).** `cargo test --locked --release -p lane -p math -p wasm-gates --features
math/lane` pass. `bash scripts/run-wasm-gates.sh` pass: native and wasm `simd128` legs both
report 144 cases (was 143), 0 mismatches.

**Other gates.** `cargo test --locked --release -p gate-expander` pass; `cargo fmt --all --
--check` pass; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
pass; `scripts/check-workspace-policy.sh` pass; `scripts/check-cross-targets.sh` PASS. Realtime
policy and the worklet chain not run: no render code and no worklet-compiled code changed (the
corpus module is not in the worklet). AArch64 runs only in CI.
