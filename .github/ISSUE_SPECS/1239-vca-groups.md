# VCA groups

The umbrella for VCA groups, decision 13's owner-delegated answer (a)
(`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`, DESIGN 2.2a). It is separate from
*Submix strips and live aux sends* (#1196): nothing there depends on VCA. It was filed when batch K3
of #1196 was delivered (*Let a send follow its source strip's mute live in the browser*, #1224), and
every slice's anchors were re-verified on the K3 head, `8c6268967`.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`. VERIFY-2's M12 (a prepared
follow-mute must include the VCA's mute) and M19 (migration gaps) are folded into the slices.

## Product outcome

A session can group strips (tracks and submixes) under a VCA: a control-only fader whose per-lane dB
offset adds to every member's own fader, and whose per-lane mute mutes every member, with no audio
path.

- Pulling a `drums` VCA down 6 dB lowers every drum track's effective fader by 6 dB, so their
  post-fader reverb sends drop with them. An audio drum submix cannot do that: its fader acts after
  the sends have left.
- Muting a VCA mutes every member, and silences every `follows_mute` send from a member, in a fresh
  plan and live alike.
- VCAs nest and overlap.
- The grouping, each member's own value and each VCA's value are saved in the canonical session, so
  a fan's phone, the next agent and the producer's mixer see the same intent.

## Why it is in scope (DESIGN 2.2a, upheld by VERIFY-1)

1. **The intent must live in the session.** The engine owns the edited session and apps save its
   canonical JSON; V1 has no free-form metadata. An agent that emulates a VCA by rewriting member
   faders has nowhere to keep the grouping or each member's own value, and once a member clamps at
   the `[-144, 24]` dB edge the saved session has lost the balance for good.
2. **The audio semantics are the industry's** (Pro Tools, S6L, DiGiCo, SSL; DESIGN section 4): dB
   offsets summed over a reach set, each VCA once; mute an implicit OR that keeps the member's own
   mute; no audio path; post-fader sends follow because the offset lands on the member's fader;
   clamping to the fader domain is the engine's form of S6L's cap.
3. **It needs no render code.** The effective gain folds into the fader coefficient the member
   already applies; the effective mute into the strip-mute state and the follow-mute inputs that
   already exist. A live move is member fader, mute and follow records through queues that exist.

## Semantics (binding on every slice)

**Schema.** A required root key `vcas: [{ id, fader: { left_db, right_db, left_mute, right_mute },
members: [id, ...] }]`, canonically after `submixes`, `[]` when empty, root field ID 16 (#1240),
edited by session opcodes `0700`-`0702` (#1241).
VCA IDs join the graph-entity namespace of tracks, submixes and outputs. A member is a track, a
submix or another VCA; membership is acyclic (`vca.cycle`). A VCA's fader values are offsets in
`[-144, 24]` dB.

**Composition**, for strip `s` and lane `l` (`l` is left or right):

1. `reach(s)` is the **set** of VCAs from which `s` is reachable through membership, directly or
   through nested VCAs. Each VCA counts once, even along several paths (a diamond).
2. **Effective gain:** `clamp(own_db(s, l) + sum(v.l_db for v in reach(s)), -144, 24)`, summed in
   `f64` in a fixed order (the member's own value, then the VCAs in ascending VCA-ID order), clamped
   in `f64`, rounded once to `f32`. With an empty reach it is the member's own value, bit for bit.
   One function computes it (#1242's `session::vca_effective_db`); every host and every slice calls
   it, never a re-spelling.
3. **VCA mute:** `vca_mute(s, l) = any(v.l_mute for v in reach(s))`.
4. **Prepared mute** (every host, at preparation): `own_mute(s, l) || vca_mute(s, l)`. It feeds both
   the strip's prepared fader section and the prepared `source_lane_muted` of every `follows_mute`
   route whose source is `s` (VERIFY-2 M12).
5. **Live effective mute in the browser** (`host_core::LiveControlSoloState::effective_mute`, the one
   composition, DESIGN P7):
   `user_mute(s, l) || vca_mute(s, l) || (any_solo && !solo_safe(s) && !solo(s))`.
   - **Mute wins.** Solo never clears a user or VCA mute: a soloed member of a muted VCA stays
     muted, and so do its following sends.
   - **Solo-safe is not VCA-safe.** A submix is solo-safe, but a VCA that reaches it mutes it.
   - **A VCA has no solo.** A VCA mute neither engages a solo nor counts toward `any_solo`.
6. **Live effective mute on the C ABI** (no solo): `own_mute(s, l) || vca_mute(s, l)` of the
   committed model. Until #1247 lands, every C ABI delta of a model that declares a VCA is structural
   (the P13 guard, below), so this is only read at preparation before then.
7. **Follow-mute** (`follows_mute`, DESIGN P11): a following route's `source_lane_muted` is its
   source strip's effective mute by rule 4 at preparation, rule 5 live in the browser and rule 6 on
   the C ABI. A route's own `mute` is independent and no VCA touches it. A send without
   `follows_mute` (a pre-fader monitor send) is not silenced by a VCA mute, exactly as it is not by
   the member's own mute; a post-fader or post-pan send is silenced through the fader.
8. **No VCA send trim.** A VCA never changes a route's `gain_db`; post-fader sends follow a VCA
   because they tap after the member's fader.

**Snapshots.** Canonical JSON stores each member's own value and each VCA's value, never the
effective value. "Coalesce" (baking a VCA into its members) is an ordinary transaction an agent
writes.

**Caps.** `HostPrepareCaps.maximum_vcas`, and on the C ABI the compile limits' `reserved[0]` (of the
three-word tail) becomes `maximum_vcas`, where 0 means "use `maximum_tracks`", mirroring
`maximum_submixes` (#1206). The struct stays 208 bytes (#1243).

**#1053 coordination (DESIGN P13, widened at filing).** #1053's live classifier pushes a member's own
fader value, and #1225's route records and #1226's follow mirror read the source strip's raw
committed mutes, so each would drop a VCA. Until *Deliver value-only VCA edits to the running C ABI
plan* (#1247) lands, a C ABI committed-model delta is **structural whenever the pre- or post-commit
model declares at least one VCA**. The filing commit amended #1053's spec (A2 D1, "Coordination"),
#1225's D1 structural list and #1226's D5 to say so. Whichever of #1053 and *Apply VCA offsets and
mutes at preparation* (#1242) lands second implements it in `live_builtin_delta`; #1247 replaces it
with composition.

## Slices

| Label | Issue | Title | Depends on |
|---|---|---|---|
| V1 | #1240 | Declare VCA groups in the session | this umbrella; #1224 (K3 delivered) |
| V2 | #1241 | Edit VCA groups through session transactions | #1240 |
| V3 | #1242 | Apply VCA offsets and mutes at preparation | #1240 |
| V4 | #1243 | Count and cap VCA groups in host preparation and the C ABI | #1240 |
| V5 | #1244 | Compose live VCA moves in host-core | #1242 |
| V6 | #1245 | Ride VCA groups live in the browser | #1244; #1224 |
| V7 | #1246 | Enumerate VCA groups and drive them from the SDK | #1245 |
| V8 | #1247 | Deliver value-only VCA edits to the running C ABI plan | #1241; #1246; #1225; #1226; #1053 (**not landed**) |

The drafts (`V0`-`V5` at `8c6268967`) were split at filing so each slice is about half a day and one
outcome: the session edits out of the grammar slice (on the #1199/#1204 precedent), the caps out of
preparation (#1206), and the host-core composition out of the browser slice (#1221/#1222). The
drafts' V1 is V1 and V2 here, their V2 is V3 and V4, their V3 is V5 and V6, their V4 is V7 and
their V5 is V8.

## Batches (CI-conscious mode)

- **VCA batch: #1240-#1246, in label order, pushed once** after #1246's verdict: the root-key
  migration (#1240), the session edits (#1241), preparation (#1242), caps (#1243), live composition
  (#1244), the browser's admission (#1245), then enumeration and the SDK (#1246).
  - Inside the batch only: between #1240 and #1242 VCAs parse and are inert; between #1242 and #1245 a
    browser kind 3 (`faderDb`) on a VCA member stages the member's own value and drops the offset.
    Neither state is pushed.
  - #1246 closes the batch and removes the decision-13 qualifier from `AGENTS.md`'s VCA sentence
    (#1197 D5). From #1242 on, VCAs apply at preparation on every host, the C ABI included, so #1247
    is not needed for that.
- **#1247** follows once #1053, #1225 and #1226 have closed and the VCA batch is on `main`.
- The VCA batch and batch C1 of #1196 both touch host-core; the root never runs both at once
  (`AGENTS.md`: one launch-critical implementation at a time).

## Out of scope

- VCA solo.
- VCA trim of send levels (SSL V-Aux, DiGiCo since v1445).
- VCA automation: an automation target naming a VCA refuses (`reference.missing_entity`); stored
  automation is inert until #1058 anyway.
- A coalesce engine operation.
- A per-VCA metadata family: a VCA fader's domain is the builtins' `fader_db` row the metadata
  already publishes (#1246).

## Objective gates (umbrella)

The umbrella closes when #1240-#1247 have each recorded Sol PASS, their evidence commits are upstream,
and their GitHub issues are closed and verified.

## Dependencies

- *Let a send follow its source strip's mute live in the browser* (#1224; batch K3 of #1196
  delivered).
- *Record the submix, send and VCA ruling* (#1197; the VCA ruling).
- #1247 also: *Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes* (#1053), *Deliver value-only send and submix-strip edits to the running C
  ABI plan* (#1225), *Let C ABI sends follow their source strip's mute live* (#1226).
