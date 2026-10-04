# Fade in a strip that a swap adds during playback

Slice 18 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

**Blocked on owner question Q3** of the umbrella (fade in added strips, and the ramp length). Do not
start before the ruling. If the owner declines, close this issue as not planned.

## Product outcome

A stem that a structural edit adds while audio plays enters with a short fade instead of a step. A
stem rarely starts at a zero crossing, so without a fade its first block is a click. Unchanged
strips are not touched: their output stays bit-identical to the reference.

## Context

- An added strip's nodes start at rest (umbrella P1). Its source is either an existing persisting
  source (it plays from the swap block) or a new source started by an anchored seek at render sample
  `A` (*Start a newly added C ABI source at an exact render sample*, #1275), silent until then.
- The fader/mute stage already has a ramp: `FaderRampStage` (`crates/builtins/src/lib.rs:2510`),
  used by live fader and mute records (`FaderMuteRampBuiltins`, `:3989`). #1053 rules the live mute
  ramp length; #1054 later moves ramp lengths into the session (`controlSmoothing`).
- The fader carry comes from *Carry fader, mute and pan ramps across a plan swap* (#1277). The
  predecessor's state inventory lists every strip ID it had.
- The graph source set reports per block whether a claim's source played a block
  (`played_planes`, `crates/source/src/lib.rs:1572`).

## Decisions to freeze after the ruling (planner's proposal)

- **D1. Which strips.** Exactly the strips whose strip ID is absent from the predecessor's inventory
  (added strips). Never a strip that existed before, even one whose stages were not carried because
  they changed; never at a first compile or a boot.
- **D2. Trigger.** The fade starts at the first block, at or after the swap block, in which the
  strip's source plays a block. So a stem anchored at `A` fades in at `A`, and a stem on a playing
  source fades in at the swap block.
- **D3. Shape and length.** The fader ramp from gain 0 to the strip's prepared fader value, over the
  length the owner rules (proposed: the live mute ramp length #1053 rules, until #1054's session
  setting exists). After the ramp the output equals the reference bit for bit (the ramp's end
  snap).
- **D4. Cost.** One prepared flag per added strip lane and the existing ramp kernel; no allocation;
  nothing on unchanged strips.

## Deliverables (after the ruling)

1. D1-D4 in `crates/builtins`, `crates/builtins-compiler`, `crates/graph` and host-core's successor
   preparation.
2. Tests (below) and the header and browser documentation of the fade.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/include/miso_engine_v1.h` (comments), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No fade-out of a removed strip (umbrella Q5, Deferred).
- No declick of an ordinary seek.

## Objective gates (proposed)

1. **Only added strips fade.** The transaction adds a strip and also changes one existing strip's
   fader, in `crates/host-core/tests/successor_swap.rs`, with slice 5's anchored-seek reference
   (an added source fed zeros before `A`). Every other strip's contribution is bit-identical to that
   reference (compare with the added strip muted in both), and the changed strip starts at its new
   value with no fade.
2. **Fade shape.** The added strip's first audible block equals the reference strip's signal times
   the ramp, sample for sample, and every block after the ramp is bit-identical to the reference.
3. **Out-of-band energy.** For a full-scale sine started at a peak, the swap block's out-of-band
   energy is at least 30 dB lower with the fade than without it (#1053's click measure).
4. **Realtime.** Zero allocations and frees in the swap block and in the fade blocks.
5. Commands: the builtins, builtins-compiler, graph and host-core test runs, and the umbrella's
   inherited gates.

## Test value

- Gate 1: a fade applied to every strip at the swap (a global dip) turns it red.
- Gate 2: a fade that starts at the swap block for an anchored stem (finished before the stem plays,
  so the click remains) turns it red.
- Gate 1: a fade keyed on "not carried" rather than "absent" dips a strip that only changed; it turns
  red when the gate also edits one existing strip's fader in the same transaction.

## Dependencies

- *Start a newly added C ABI source at an exact render sample* (#1275).
- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- Owner question Q3 of *Swap a rebuilt plan without an audio gap* (#1269).
