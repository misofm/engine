# effect-contract: a bypass switch during a warm-up hold crossfades from wet

Tracker #2258, item GR-3 (bypass crossfade lane phase). Found by the implicit-state-machine audit
of 2026-11-06.

## Problem
`EffectControlLane` (`crates/effect-contract/src/live.rs`) kept its bypass crossfade phase in two
loose words, `mix_position` and `hold`. `stage` started a switch's crossfade from
`mix.coefficients_at(mix_position)` and ignored `hold`. Take a lane that leaves settled bypass by a
**step** (ramp `0`). Its `mix` is the settled step, so `coefficients_at(0)` is the target `1`
(wet), while issue #2129 D3's hold makes it emit dry for the effect's warm-up. Bypass it again,
with a ramp, before the hold ends: the new crossfade runs from `1`. The output jumps from the dry
signal to the reset effect's warm-up output, which is exactly what the hold exists to keep out.
`bypass_mix()` also reported `1` during such a hold.

**Reachable today.** The hold spans a block boundary whenever the effect's warm-up exceeds the
frames left in the switch's block. The render quantum is caller-supplied, and the warm-up is the
declared latency or more: the limiter's lookahead is a parameter, and soft-clip's is latency plus
tail. Both the per-node path (`graph::runtime::LiveControlEffect::stage`) and the bank path
(`rack::LiveControlEffectBankStage::read_cells`) stage through this function, so both rendered
the jump, and they agreed with each other.

The tracker's other two points, checked:
- **Two encodings.** The bank's negative `f32` position was already derived from
  `(mix_position, hold)` in one place (`bypass_crossfade_lane`). It was not a second stored
  state. Its soundness rested on an unstated invariant: `hold != 0` implies `mix_position == 0`.
  `begin_hold` was `pub` and unchecked, so any caller could break that invariant. The real call
  sites did not.
- **Call order.** `take_rest`, then reset, then `begin_hold`. Both production call sites
  (`crates/graph/src/runtime.rs` and `crates/rack/src/lib.rs`) and the test re-enactment in
  `crates/graph-compiler/tests/bypass_shunt_identity.rs` follow it. Nothing enforced it.

## Decision / model
- **`CrossfadePhase` replaces `(mix_position, hold)`.** It is a private enum, `Held { frames:
  NonZeroU32 }` or `Ramp { position }`, so a hold beside a moved ramp cannot be represented. This
  is per-lane control state, read once per block, not a SIMD lane word.
- **One function per derivation.** `CrossfadePhase::mix` gives the mix the lane emits: `0` while
  held. `stage` starts every switch from it, and `bypass_mix()` returns it. This is the fix.
  `CrossfadePhase::bank_position` derives the bank kernel's lane word, `-frames` or `position`.
  `CrossfadePhase::advance` spends a hold before the ramp moves.
- **The bank kernel is unchanged.** `BankCrossfadeLane`'s `f32` words and
  `BypassShunt::crossfade_bank`'s branch-free `k < 1` select stay as they are. Per the tracker's
  rule, SIMD lane words do not become enums. The bank encoding is legitimate: it is now derived
  from the explicit phase through one function.
- **`RestToken`.** `take_rest` returns `Option<RestToken>`; both it and `RestToken` are
  `#[must_use]`. `begin_hold(rest, frames)` consumes the token, so a hold can only follow a
  `take_rest` that reported a lane leaving settled bypass. As validation, `begin_hold` holds only
  a lane that is still unbypassed and at its ramp's first frame (debug-asserted). The reset
  between the two calls belongs to the caller's processor and stays documented. Three call sites
  changed.
- **Carry.** `carry_from` copies the phase as one value. Every phase value is legal, so a carried
  or swapped lane needs no further validation.

## Non-goals
No change to `BypassShunt`'s kernels or `BankCrossfadeLane`, to any effect crate, or to
`crates/true-peak-limiter`. No change to the switch semantics of #2129 or #1341 beyond the start
mix of a switch made during a hold.

## Gates
- A regression test red on base, green at head.
- No-bit-moved comparison against base for every path that does not switch during a step-started
  hold. This is PR evidence, not a committed digest.
- `cargo test --locked -p effect-contract -p rack -p graph -p graph-compiler`; clippy
  `-D warnings`; `cargo fmt --check`; `bash scripts/check-workspace-policy.sh`; the realtime
  policy tool.

## Evidence (attempt 1)

**Red on base** (`de0878e23`). New test
`live_control::a_bypass_during_a_hold_crossfades_from_the_dry_the_hold_emits`
(`crates/effect-contract/tests/live_control.rs`), written against base's `bool` `take_rest`:
```
assertion `left == right` failed: a held lane renders dry
  left: 1.0
 right: 0.0
```
With the accessor asserts commented out, the rendered block after the switch fails too. Its
left words are about `-0.5` (the wet input), where the dry input of about `+0.25` is expected:
```
assertion `left == right` failed
  left: [3204251648, 3204179968, ...]   (wet, about -0.5)
 right: [1048576000, 1048838144, ...]   (dry, 0.25 ...)
```
It passes at head.

**Test value.** The test turns red if a switch during a hold reads the mix of the un-bypass ramp
it interrupts instead of the dry the hold emits. That reproduces the jump. No existing test
catches this: the hold tests in `bypass_skip_warmup.rs` and `bypass_shunt_identity.rs` never
switch during a hold that a step started, and the bank-equals-node tests compare two paths that
shared the bug.

**Mutations.**
- Restoring `now = mix.coefficients_at(position)` in `stage`: the new test fails
  (`the new ramp starts at the hold's dry`, `1.0` vs `0.0`).
- `bank_position` returning `0.0` for a hold:
  `bypass_shunt_identity::a_bank_crossfades_every_lane_exactly_as_the_node_does` fails.
- `begin_hold`'s `fresh` guard is unreachable by construction, since the token is issued only
  after `stage` sets `Ramp { position: 0 }` on an unbypassed lane. No test can turn it red. It is
  hardening, debug-asserted.

**No bit moved** (one-time, scratch, not committed). Two scratch digests, run on base and at head.
Each FNV-1a hash covers every output word of a lane over every block.
1. Effect-contract kernels: 200 seeds, 60 blocks of 32 frames each. Bank widths `f32`, `Simd4`
   and `Simd8` give 2,600 lanes. Each lane gets random warm-ups (0 to 150), random starts, and
   random switches whose ramps are `{0, 5, 17, 32, 64, 100, 300}`. The bank path runs
   `crossfade_bank` over `bypass_crossfade_lane`, and the per-node path runs `apply_bypass`. The
   ramping, needs-dry and settled flags are hashed per block. A lane is flagged if it was bypassed
   within its warm-up after a step un-bypass (an over-approximation of the defect's trigger).
   - All 2,015 unflagged lanes are identical, bank and node.
   - 425 of the 585 flagged lanes differ, as intended.
2. Real effects through `rack::LiveControlEffectBankStage` and the per-node re-enactment: all nine
   bankable launch effects, 6 seeds, blocks of 32 and of 128 frames, 48 blocks each, at host width
   (8 lanes), 864 lanes in total.
   - All 788 unflagged lanes are identical.
   - 35 of the 76 flagged lanes (soft-clip and limiter) differ, as intended.
   - Bank equals node in every lane, on base and at head.

`bypass_mix()` also moves during a step-started hold (`1` becomes `0`). That is the documented
accessor fix.

**Gates (head).**
- `cargo test --locked -p effect-contract -p rack -p graph -p graph-compiler`: 52 test binaries,
  510 tests, all pass.
- `cargo clippy --locked -p effect-contract -p rack -p graph -p graph-compiler --all-targets --
  -D warnings`: clean.
- `cargo check --locked --workspace --all-targets`: builds. Its one warning is in host-web
  (`set_state_for_test`), which this change does not touch.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `./target/release/realtime_policy`: `realtime policy: ok (292 marked regions in 46 files)`.

**Open.** `BypassShunt::crossfade_held` keeps its `position` parameter. Its only caller now passes
`0`, so a follow-up could drop it. That follow-up would change a kernel signature and nothing
else.
