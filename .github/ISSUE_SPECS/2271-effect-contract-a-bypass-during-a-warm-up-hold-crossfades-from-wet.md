# effect-contract: a bypass switch during a warm-up hold crossfades from wet

Tracker #2258, item GR-3 (bypass crossfade lane phase). Found by the implicit-state-machine audit
of 2026-11-06.

## Problem
`EffectControlLane` (`crates/effect-contract/src/live.rs`) kept its bypass state in three loose
fields: `bypass`, `mix_position` and `hold`. A switch to bypass made during issue #2129 D3's
warm-up hold was handled wrongly in two ways. On base, both are reachable on the per-node and the
bank path.

1. **The switch crossfaded from wet.** `stage` started the switch's crossfade from
   `mix.coefficients_at(mix_position)` and ignored `hold`. Take a lane that leaves settled bypass
   by a **step** (ramp `0`). Its `mix` is the settled step, so `coefficients_at(0)` is the target
   `1` (wet), while the hold makes it emit dry for the effect's warm-up. Bypass it again, with a
   ramp, before the hold ends: the new crossfade runs from `1`. The output jumps from the dry
   signal to the reset effect's warm-up output, which is exactly what the hold exists to keep out.
   `bypass_mix()` also reported `1` during such a hold.
2. **A reversal escaped the warm-up** (found by the attempt-1 review). The same switch threw away
   the warm-up the lane still owed and left it *ramping* toward bypass, not settled there. An
   un-bypass during that ramp is a reversal: `take_rest` reports nothing, so the lane gets no
   reset and no hold, and it crossfades straight onto an effect fewer than its warm-up frames past
   its reset. That breaks #2129's rule that every un-bypass from settled bypass resumes from rest.
   On base the reversal starts from about `0.5`; on attempt 1 it starts from `0`. Both are wrong,
   because the effect is still warming up.

**Reachable today.** The hold spans a block boundary whenever the effect's warm-up exceeds the
frames left in the switch's block. The render quantum is caller-supplied, and the warm-up is the
declared latency or more: the limiter's lookahead is a parameter, and soft-clip's is latency plus
tail. Both the per-node path (`graph::runtime::LiveControlEffect::stage`) and the bank path
(`rack::LiveControlEffectBankStage::read_cells`) stage through this function. They rendered the
same jumps, so they agreed with each other.

The tracker's other two points, checked:
- **Two encodings.** The bank's negative `f32` position was already derived from
  `(mix_position, hold)` in one place (`bypass_crossfade_lane`). It was not a second stored
  state. Its soundness rested on unstated invariants: `hold != 0` implies `mix_position == 0`
  and `!bypass`. `begin_hold` was `pub` and checked only `!bypass`, so any caller could break
  the first invariant. The real call sites did not.
- **Call order.** `take_rest`, then reset, then `begin_hold`. Both production call sites
  (`crates/graph/src/runtime.rs` and `crates/rack/src/lib.rs`) and the test re-enactment in
  `crates/graph-compiler/tests/bypass_shunt_identity.rs` follow it. Nothing enforced it.

## Decision / model
- **`BypassPhase` replaces `(bypass, mix_position, hold)`.** It is a private enum:
  `Held { frames: NonZeroU32 }` or `Ramp { position, bypass }`. A hold therefore cannot sit beside
  a moved ramp or a bypass target, because a held lane always heads wet. This is per-lane control
  state, read once per block, not a SIMD lane word. `size_of::<EffectControlLane>()`, which
  `graph-compiler/src/estimate.rs` charges, stays at 304 bytes, as on base.
- **A switch to bypass during a hold settles the lane at bypass at once.** It sets
  `mix = settled_mix(true)` and the phase to the start of a ramp toward bypass. This is the
  reviewer's option (a). The lane already emits dry, so the output stays the dry signal, now as an
  exact copy of every word. Its next un-bypass leaves settled bypass and takes rest: a reset and a
  whole hold. Every other switch starts its ramp from `BypassPhase::mix`, the mix the lane emits
  now, as before.
- **One function per derivation.** `BypassPhase::mix` is the mix the lane emits (`0` while held),
  and `bypass_mix()` returns it. `BypassPhase::bank_position` derives the bank kernel's lane word,
  `-frames` or `position`. `BypassPhase::advance` spends a hold before the ramp moves.
- **The bank kernel is unchanged.** `BankCrossfadeLane`'s `f32` words and
  `BypassShunt::crossfade_bank`'s branch-free `k < 1` select stay as they are. Per the tracker's
  rule, SIMD lane words do not become enums. The bank encoding is legitimate: it is now derived
  from the explicit phase through one function.
- **`BypassShunt::crossfade_held` loses its dead parameters.** Its only caller always passed
  `position = 0`, and `settled_dry = false` because a held lane heads wet. The kernel now fixes
  both, with the same arithmetic.
- **`RestToken`.** `take_rest` returns `Option<RestToken>`; both it and `RestToken` are
  `#[must_use]`. `begin_hold(rest, frames)` consumes the token, so a hold can only follow a
  `take_rest` that reported a lane leaving settled bypass. As validation, `begin_hold` holds only
  a lane at `Ramp { position: 0, bypass: false }` (debug-asserted).
  - The token is not tied to a lane. Naming the lane would cost a word per token and a comparison
    per hold, to rule out a mistake both call sites already exclude by their shape. A misplaced
    token still holds only an un-bypassed lane at its ramp's first frame.
  - The reset between the two calls belongs to the caller's processor and stays documented.
  - Three call sites changed.
- **Carry.** `carry_from` copies the phase as one value. Every phase value is legal, so a carried
  or swapped lane needs no further validation.

## Sound quality
The objective gate is exact dry bits through the switch and through the hold that follows a
reversal. The defect is a discontinuity that is removed outright, not attenuated or traded off,
so a blinded listening test adds no evidence here.

## Non-goals
No change to `BypassShunt::crossfade`, `crossfade_bank` or `BankCrossfadeLane`, to any effect
crate, or to `crates/true-peak-limiter`. No change to the switch semantics of #2129 or #1341
beyond a switch made during a hold.

## Gates
- A regression test for each defect, red on base, green at head. The reversal test is also red on
  attempt 1.
- No-bit-moved comparison against base for every lane that is never bypassed during a hold. This
  is PR evidence, not a committed digest.
- `cargo test --locked -p effect-contract -p rack -p graph -p graph-compiler`; clippy
  `-D warnings`; `cargo fmt --check`; `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps` on the
  touched crates; `bash scripts/check-workspace-policy.sh`; the realtime policy tool.

## Evidence (attempt 1): FAIL
Commit 48608d056. The fix covered defect 1 only.
- Its test was `a_bypass_during_a_hold_crossfades_from_the_dry_the_hold_emits`, red on base with
  `a held lane renders dry  left: 1.0 right: 0.0`. With the accessor asserts removed, the block
  after the switch rendered wet (about `-0.5`) where dry (`+0.25`...) was expected.
- The adversarial review agreed with the narrow fix, the no-bit-moved classification and the
  SIMD/banking analysis.
- It failed on one MAJOR, defect 2. Repro at attempt 1: step un-bypass, `begin_hold(3 * 64)`, one
  block, bypass with `ramp_samples = 128`, one block, `BypassSwitch::step(false)`. `take_rest()`
  is `None`, and the next block is fully wet, 128 frames after the reset, against a warm-up of
  192.
- MINORs: `Held` with `bypass == true` was representable; `crossfade_held`'s `position` was dead;
  the `RestToken` doc did not say the token is untied to a lane; the spec had no sound-quality
  sentence. All are addressed in attempt 2.

## Evidence (attempt 2)

**Tests** (`crates/effect-contract/tests/live_control.rs`). They share a `Held` fixture: a lane
that left settled bypass by a step, took its rest and has held dry for one block of a three-block
warm-up.
- `a_bypass_during_a_hold_renders_the_dry_the_hold_emits` (defect 1, attempt 1's test on the
  fixture). A bypass with a ramp mid-hold reports no rest, keeps `bypass_mix() == 0`, renders
  three blocks of exact dry bits, and is settled at bypass.
  - *Test value:* red if the switch crossfades from the interrupted step's mix (`1`) instead of
    the hold's dry. No existing test switches during a hold that a step started.
- `an_un_bypass_after_a_bypass_during_a_hold_resumes_from_rest` (defect 2). After that switch, a
  step un-bypass two blocks past the reset must report its rest. The lane then holds dry for a
  whole warm-up again: three blocks, past the end of the first hold. Only then does it reach wet.
  - *Test value:* red if a bypass during a hold leaves the lane ramping rather than settled: the
    un-bypass would report no rest and get no reset or hold, and would crossfade onto an effect
    still warming up. No existing test reverses a switch made during a hold.

**Red on base** (`de0878e23`), with the fixture written against base's `bool` `take_rest`:
- Both tests stop at the fixture's accessor assert: `a held lane renders dry  left: 1.0 right: 0.0`.
- With that assert off, the first test fails with
  `the switch keeps the hold's dry  left: 1.0 right: 0.0`. The second fails at the block after the
  switch (`assertion failed: held.block_is_dry()`): the jump of defect 1.
- With that assert off too, it fails with `left settled bypass` from `take_rest`: defect 2.

**Red on attempt 1** (48608d056 `live.rs`, attempt-2 tests). The first test passes. The second
fails with `left settled bypass` (`live_control.rs:556`): the un-bypass reports no rest.

**Green at head:** `live_control`, 19 passed.

**Mutations at head.**
- The held branch of `stage` ramping `0 -> 0` (attempt 1's behaviour) instead of
  `settled_mix(true)`: the reversal test fails (`left settled bypass`), and the first test passes.
- The held branch crossfading from `mix.coefficients_at(0)` (base's start mix): both tests fail
  (`the switch keeps the hold's dry`, `1.0` vs `0.0`; `held.block_is_dry()`).
- `bank_position` returning `0.0` for a hold (from attempt 1, kernel unchanged since):
  `bypass_shunt_identity::a_bank_crossfades_every_lane_exactly_as_the_node_does` fails.
- The `fresh` guard in `begin_hold` cannot be reached by construction, so no test can turn it
  red. It is hardening, debug-asserted.

**No bit moved** (one-time, scratch, not committed). Both digests were re-run on base and at
head, with the trigger flag widened to cover defect 2. A lane is now flagged if any switch to
bypass lands within its warm-up after *any* un-bypass, whatever the un-bypass's ramp. That is an
over-approximation of "bypassed during a hold", and a flagged lane's hash covers everything that
follows, so a later reversal is covered too. Each FNV-1a hash covers every output word of a lane
over every block. The flags are computed from the schedule alone, and they matched between base
and head.
1. **Effect-contract kernels:** 200 seeds, 60 blocks of 32 frames, bank widths `f32`, `Simd4` and
   `Simd8`, 2,600 lanes in total.
   - Each lane gets random warm-ups (`0` to `150`), random starts, and random switches with ramps
     of `{0, 5, 17, 32, 64, 100, 300}`.
   - The bank path runs `crossfade_bank` over `bypass_crossfade_lane`, and the per-node path runs
     `apply_bypass`. The ramping, needs-dry and settled flags are hashed per block.
   - All 1,694 unflagged lanes are identical, on the bank and the node path.
   - 705 of the 906 flagged lanes differ, as intended.
2. **Real effects** through `rack::LiveControlEffectBankStage` and the per-node re-enactment: all
   nine bankable launch effects, 6 seeds, blocks of 32 and of 128 frames, 48 blocks each, at host
   width (8 lanes), 864 lanes in total.
   - All 736 unflagged lanes are identical.
   - 111 of the 128 flagged lanes differ, as intended; all flagged lanes are soft-clip (34) and
     limiter (94).
   - Bank equals node in every lane, on base and at head.

`bypass_mix()` also moves during a step-started hold (`1` becomes `0`). That is the documented
accessor fix.

**Gates (head).**
- `cargo test --locked -p effect-contract -p rack -p graph -p graph-compiler`: 52 test binaries,
  511 tests, all pass.
- `cargo clippy --locked -p effect-contract -p rack -p graph -p graph-compiler --all-targets --
  -D warnings`: clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p effect-contract -p rack -p graph
  -p graph-compiler`: clean.
- `cargo check --locked --workspace --all-targets`: builds. Its one warning is in host-web
  (`set_state_for_test`), which predates this change and is not touched by it.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.
