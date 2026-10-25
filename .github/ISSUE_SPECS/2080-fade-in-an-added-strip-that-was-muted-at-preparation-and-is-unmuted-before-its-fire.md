# Fade in an added strip that was muted at preparation and is unmuted before its fire

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #2069. The cited closed spec is read
at `da15a2824` (it has left this folder).

## Problem
A plan swap arms a fade-in on each strip it adds during playback (#1288 D2): each channel waits
muted and opens at the fire over the session mute ramp. A channel the successor session mutes at
preparation is not armed. If a live record unmutes that channel between the swap block and the
fire, the strip opens at once over the record's ramp, not at the fire over the fade. A strip muted
on both channels arms nothing and gets no fade entry, so nothing reports it. #2069 fixed the same
defect for routes; the strip fader keeps it.

## Evidence
- `crates/builtins/src/lib.rs:3237-3251` (fader stage `arm_fade_in`): a channel with
  `muted[channel][lane]` set is "left as it is" (not armed).
- `crates/builtins/src/lib.rs:3355-3366` (`mute_channel`): only an armed channel keeps its arm on an
  unmute; an unarmed channel unmutes at once over the record's window.
- `crates/graph/src/lib.rs:4669-4675` (`install_fade_arms` doc): "A strip the session mutes on both
  channels arms nothing and gets no entry."; `crates/graph/src/lib.rs:4731-4750`: a fader arm that
  armed nothing pushes `None`.
- `crates/host-core/src/prepare.rs:8269-8278`: the host builds a `GraphFadeTarget::Fader` arm per
  added strip.
- #2069 spec decision record, "Successor (not fixed here)".

## Smallest slice
Apply #2069's rule to the strip fader: a channel muted at preparation arms too (muted, its fire goes
to its prepared mute), so an unmute record before the fire keeps the arm and the channel opens at
the fire over the fade; a fire with the mute still set ramps nowhere (no work on silence). The
strip gets its fade entry and carry pair even when both channels are muted. Record the rule and its
reason in this spec. If the rule changes `mute_channel`'s unmute-keeps-arm semantics, say how.

## Gates
- One builtins or graph test: a strip added muted, unmuted by a record before the fire, renders
  equal to the same strip added unmuted (fade at the fire), on every width the build binds; red on
  the current code (PR evidence).
- No rendered bit moves for a strip that is not muted at preparation (one PR comparison, not a
  committed test).
- `cargo test -p builtins`, `-p graph` and `-p host-core` pass.

## Paths
`crates/builtins/src/lib.rs`, `crates/graph/src/lib.rs`, `crates/host-core/src/prepare.rs`, their
tests; this spec.

## Decision record (attempt 1, 2026-10-25)
- R1 (root): the arm persists through mute and unmute records until the fire, the rule #2069 gave
  routes. It supersedes #1288 attempt 2's ruling "a user's mute wins over the fade" (a mute
  disarmed). Reason: the model holds the final value and the fade is how an added strip reaches
  it; a mute never needs the arm to protect it, as the armed lane is already held at `+0.0`.
- Builtins (`FaderRampStage`): a new `opens` bit per channel lane. `arm_fade_in` arms both
  channels; a muted one gets `opens = false`, and the call returns `true`. `mute_channel` on an
  armed channel only sets `opens = !muted`. `fire_fade_in` disarms and unmutes over the ramp only
  if `opens` is set; else it does nothing (no work on silence). `FaderChannelState` carries
  `opens`; it is in memory only, not a wire format, and no digest pins it.
- `is_armed` (public, bank and scalar) now means "armed and the fire opens it". Its only
  consumers (`builtins-compiler`'s `FaderJumpStage::muted`, `bank_hold` and the scalar hold) read
  `is_muted && !is_armed` as the model's mute. With this meaning that stays exact: an armed channel
  whose fire stays muted reports muted, so a curve's unmute before the fire reaches the stage and
  retargets the fire.
- Graph: no logic change. The fader arm now returns `true`, so a muted strip gets a fade entry
  and its carry pair. The `None` skip in host-core stays for an arm that armed nothing.
- Bits move only for a mute-then-unmute record sequence before the fire, on an added strip or on
  a strip muted at preparation. No fixture exercises this sequence. Evidence (PR evidence, not a
  committed test): base 609ccf399 and branch 5adcce571 each ran the same `successor_swap.rs`
  with the #2069 digest dump in `fade_run`, and the five arm-count assertions below changed into
  a non-failing print. 125 passed in both; the 185 digests are identical (sha256 `11881beb...744d`).
  The record is in `scratchpad/b2080/` (`METHOD.txt`, `base.txt`, `branch.txt`,
  `successor_swap.dump.rs`).
- Test (R2): `fader_ramp.rs` `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` adds the
  case `(user_mute, user_unmute) = (true, true)` and a lane muted before the arm, on all four
  record cases. Red on base 609ccf399: the case "mute then unmute" and the two cases "muted at
  the arm, then unmute" fail on rendered bits; the other five pass unchanged.
- Open: five `crates/host-core/tests/successor_swap.rs` assertions pin the old rule "a muted
  strip arms nothing". Each one now sees one more arm, from a muted strip. That file is outside
  this scope's paths.
