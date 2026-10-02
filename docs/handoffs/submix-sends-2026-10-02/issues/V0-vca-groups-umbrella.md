# VCA groups

A separate umbrella from *Submix strips and live aux sends*. **The root files it when batch K3 of
that umbrella closes** (*Let a send follow its source strip's mute live in the browser* has its Sol
PASS and K3 is pushed), and re-verifies every anchor in this umbrella's slices against `main` then.
The ruling that keeps VCA in scope is recorded by *Record the submix, send and VCA ruling*
(`DESIGN.md` 2.2a, upheld by VERIFY-1).

V1 is implementation-ready as drafted. V2-V5 are drafted with every gap VERIFY-2 found closed; their
anchors were read on `fe8ac679` and must be re-read at filing, because K1-K3 move them.

## Product outcome

A session can group strips (tracks and submixes) under a VCA: a control-only fader whose per-lane dB
offset adds to every member's own fader, and whose mute mutes every member, with no audio path.

- Pulling a "drums" VCA down 6 dB lowers every drum track's effective fader by 6 dB, so their
  post-fader reverb sends drop with them. An audio drum submix cannot do that: its fader acts after
  the sends have left.
- Muting a VCA mutes every member and silences every `follows_mute` send from a member, in a fresh
  plan and live alike.
- VCAs nest and overlap.
- The grouping, each member's own value and each VCA's value are saved in the canonical session, so
  a fan's phone, the next agent and the producer's mixer all see the same intent.

## Why it is in scope (`DESIGN.md` 2.2a)

1. **The intent must live in the session.**
   - The engine owns the edited session on every platform, and apps save the engine's canonical
     JSON.
   - V1 has no free-form metadata.
   - An agent that emulates a VCA by rewriting member faders has nowhere to keep the grouping, or
     each member's own value.
   - Once a member clamps at the `[-144, 24]` dB edge, the saved session has lost the balance for
     good.
2. **The audio semantics are the industry's** (Pro Tools, S6L, DiGiCo, SSL).
   - dB offsets are summed over a reach set, and each VCA counts once.
   - Mute is an implicit OR that keeps the member's own mute.
   - There is no audio path.
   - Post-fader sends follow, because the offset lands on the member's fader.
   - Clamping to the fader domain is the engine's form of S6L's cap.
3. **It needs no render code.** The effective gain folds into the fader coefficient the member
   already applies, and the effective mute into the strip-mute state and the follow-mute inputs that
   already exist. A live move is member fader, mute and follow records through queues that already
   exist.

## Semantics

- **Schema.** `vcas: [{ id, fader: { left_db, right_db, left_mute, right_mute }, members: [id...] }]`
  is a required root key, canonically after `submixes`, `[]` when empty, at the next unallocated root
  field ID (16 if still free; never 8, the unrecorded gap of the removed `limits` key).
  - VCA IDs join the graph-entity namespace.
  - Members are tracks, submixes or other VCAs.
  - Membership is acyclic (`vca.cycle`).
- **Reach.** `reach(strip)` is the **set** of VCAs from which the strip is reachable through
  membership. Each VCA counts once, even along several paths.
- **Effective gain per lane.** `clamp(member_db + sum(v.lane_db for v in reach), -144, 24)`.
  - It is summed in `f64` in a fixed order: the member, then the VCAs by ascending ID.
  - It is rounded once to `f32`, then converted by the existing `db_gain`.
  - With an empty reach it is today's value, bit for bit.
- **Effective mute per lane.** `member_mute || any(v.lane_mute for v in reach)`.
- **One computation, two consumers** (VERIFY-2 M12). One session-level helper computes each strip's
  effective fader and mute once. It feeds both:
  - the member's prepared fader section (the builtins compiler's `strip_parameters`), and
  - the prepared `source_lane_muted` of every `follows_mute` route whose source is that strip (the
    graph compiler's route lowering, through `graph_compiler::route_coefficients`).

  So a fresh plan of a VCA-muted member's follow send is silent, exactly as the live path makes it.
- **Strip mute ownership.** One per-strip mute state (`host_core::LiveControlSoloState`, sized per
  strip, with solo-safe submix entries; `DESIGN.md` P7) owns every strip's mute. VCA mute joins it as
  a per-lane `vca_mute` input:
  `(user_mute || vca_mute) || (any_solo && !solo_safe && !soloed)`. It reaches tracks **and**
  submixes, and `effective_mute(strip, lane)` stays the only composition, which the live
  follow-mute path (`LiveRouteMuteFollow::delta`) already reads.
- **#1053 coordination (`DESIGN.md` P13).** Until *Deliver value-only VCA edits to the running C ABI
  plan* lands, a committed-model delta that changes any fader field of a VCA member is **structural**
  on the C ABI. Otherwise #1053's live path would push the member's own value and drop the VCA offset.
  *Apply VCA offsets and mutes at preparation* implements the guard if #1053 has landed; the C ABI
  slice removes it.
- **Caps.** host-core gets `HostPrepareCaps.maximum_vcas`. The C ABI gets
  `miso_engine_v1_compile_limits.reserved[1]` as `maximum_vcas`, where 0 means "use
  `maximum_tracks`". This mirrors `maximum_submixes` (*Count and cap submix strips in host
  preparation and the C ABI*). The struct stays 208 bytes.
- **Snapshots.** Canonical JSON stores member values and VCA values separately, never the effective
  value. "Coalesce" is an ordinary transaction an agent writes.

## Slices

| # | Title | Depends on |
|---|---|---|
| V1 | Declare VCA groups in the session | this umbrella filed (after K3) |
| V2 | Apply VCA offsets and mutes at preparation | V1 |
| V3 | Ride VCA groups live in the browser | V2 |
| V4 | Enumerate VCA groups and drive them from the SDK | V3 |
| V5 | Deliver value-only VCA edits to the running C ABI plan | V4; *Let C ABI sends follow their source strip's mute live*; #1053 |

V1-V4 are one batch, pushed once: one root-key migration, preparation, the browser's admission, then
the browser's enumeration and SDK surface. VCAs parse but are inert between V1 and V2, inside the
batch only. V5 follows once its dependencies have closed.
The VCA batch and batch C1 of the bus and send umbrella both touch host-core; the root never runs
both at once (`AGENTS.md`: one launch-critical implementation at a time).

## Out of scope

- VCA solo.
- VCA trim of send levels (SSL V-Aux, DiGiCo since v1445).
- VCA automation: stored automation is inert until #1058.
- A coalesce engine operation.

## Objective gates (umbrella)

The umbrella closes when V1-V5 have each recorded Sol PASS, their evidence commits are upstream, and
the GitHub issues are closed and verified.

## Dependencies

- *Let a send follow its source strip's mute live in the browser* (batch K3 of *Submix strips and
  live aux sends* closed and pushed).
- *Record the submix, send and VCA ruling* (the VCA ruling).
