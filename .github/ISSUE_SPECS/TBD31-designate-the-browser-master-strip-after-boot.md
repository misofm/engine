# Designate the browser master strip after boot

Successor of TBD30 (an empty session boots), decision 17. Code anchors verified on the batch 13a
branch `codex/controls-batch-13aw`. Issue number: assigned at filing (written here as TBD31).
Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An app that opens an empty mixer can still have a designated master strip: the designation is a
control the app sets (or changes) after boot, through the one core path, instead of a boot option
that must name a strip that exists at boot.

## Context

- The browser boot option `liveControlMasterTrackPlusOne` (with observation taps) designates a
  master strip at boot; over an empty session the boot is refused `host.observation.master_track`,
  because the strip does not exist yet (TBD30's decision record).
- Since TBD30 an empty session boots on both hosts and grows by structural transactions.

## Decisions to take (the implementer proposes, the coordinator rules)

- Where the designation lives: a session-scoped control row (a setting) the core owns and both
  hosts read, or a host option that may name a strip added later (resolved at each plan's
  preparation).
- What a designation naming a strip that does not exist yet does (refused before commit, or held
  until the strip exists).

## Objective gates

1. Boot empty, add a track, designate it master after boot; the master's observation behaves as a
   boot-time designation does today, on the browser (and on the C ABI if the row is core-owned,
   with a parity row).
2. host-web and control-plane tests; the browser qualification.

## Dependencies

- TBD30.

## Size

Half a day.
