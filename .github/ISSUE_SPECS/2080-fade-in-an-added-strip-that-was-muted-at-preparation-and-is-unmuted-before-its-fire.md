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
