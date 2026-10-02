# Submix strips, sends and VCA groups: owner rulings of 2026-10-02

Follows decision 12 (`engine-footprint-2026-09-29.md`), recorded from the owner's answers on 2026-10-02.

## Decision 13: Submix strips, live aux sends and VCA groups (#1196)

The umbrella issue is *Submix strips and live aux sends* (#1196,
`.github/ISSUE_SPECS/1196-submix-strips-and-live-aux-sends.md`). It turns a submix from a bare
summing node (`Submix { id }`, `crates/session/src/model.rs:577`) into a full strip, and gives
routes a mute, a follow-mute and live levels. Today routes have no mute, and their gain is a
bind-time constant (`crates/graph/src/runtime.rs:22-24`). The planner's design and decision record
is `docs/handoffs/submix-sends-2026-10-02/DESIGN.md` (revision 2, written against `fe8ac679`).
Three Sol adversarial reviews, by verifiers who wrote none of the plan, checked it against
`fe8ac679`:

- `VERIFY-1.md` returned FAIL as written, but upheld both delegated decisions and the ramp law,
  with amendments;
- `VERIFY-2.md` returned PASS-WITH-FIXES on revision 1, and every fix is folded into revision 2
  (`REVISION-1.md`, `REVISION-2.md`);
- `VERIFY-3.md` returned PASS-WITH-FIXES on revision 2, and `APPLIED-3.md` applies every finding.
  The applied fixes have not had a fourth review.

All of these documents are in the same handoff folder.

The owner answered on 2026-10-02. The bullets below say, for each point, whose authority it carries:
the owner's own decision, the owner's delegation, or the planner's. Ruling:

- **Owner decision: submixes are strips** (DESIGN 2.1, O1). Every submix carries every session
  console slot, with its own `{slot, bypass, params}` per slot, plus ordered inserts, a per-lane
  fader and mute, and `pan` or `matrix`. Its input is the master-plan D9 sum (stable edge-ID order,
  left to right) of the routes that target it. Decision 12 applies to a submix exactly as to a
  track: its console slots always bank, padded; a bypassed lane stays in its bank; latency is
  always paid; banking couples cost, never bits.
- **Owner decision: reverb is out of scope** (O2). It is held for separate scoping. `miso.delay` is
  the send effect in tests. It declares zero latency, so PDC fixtures use the true-peak limiter.
- **Owner direction recorded in DESIGN 2.1** (O4, O5). Sends are for agents, so there are no
  human-UX limits: arbitrary bus counts, nesting, any tap, a full 2x2 per send, and fan-out. The
  graph stays acyclic, and only configured resources bound it, never a compiled maximum. SIMD is a
  core philosophy (DESIGN section 6).
- **Delegated by the owner** (O3). The owner delegated two questions, VCA groups and the submix
  strip's channel model and input section, and said they defer to the adversarially verified
  opinion. VERIFY-1 upheld both opinions, and its amendments are folded in. The two answers below
  therefore carry the owner's delegated authority.
  - **(a) VCA groups are in scope** (DESIGN 2.2a).
    - A VCA is a control-only group. It carries no audio. Its per-lane dB offset adds to each
      member's own fader, and its mute ORs into each member's effective mute.
    - It is its own umbrella, *VCA groups*, drafted at `docs/handoffs/submix-sends-2026-10-02/issues/V0`-`V5`.
      The root files it once batch K3 of #1196 closes (the Sol PASS and push of *Let a send follow
      its source strip's mute live in the browser*, #1224), and re-verifies its anchors then.
    - Only its C ABI slice (V5) also waits on #1053 and on *Let C ABI sends follow their source
      strip's mute live* (#1226). Nothing in #1196 depends on VCA.
    - Out of its scope: VCA solo, VCA trim of send levels, and VCA automation.
  - **(b) A submix strip is the same dual-mono strip a track has** (DESIGN 2.2b). L and R have
    independent state and parameters; channels link only through a declared `link_mode`; and
    cross-channel flow is only the explicit 2x2 or pan. It carries all five input-section keys
    (`polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz`, `delay_samples`). A nonzero submix
    `delay_samples` delays the summed input, and PDC never compensates it. A submix never takes the
    mono collapse: the collapse eligibility set stays tracks-only, and that set is the guard.
    Hazard: a console compressor declared `dual_mono` runs unlinked on every bus. The guidance is to
    bypass it on buses and use a linked insert for bus glue.
- **Planner decisions P1-P17** (DESIGN 2.3). These are the planner's decisions. Adversarial
  verification (VERIFY-1 to VERIFY-3) upheld them, and they are **subject to owner review**: the
  owner has not reviewed them one by one. The slices implement them as written until the owner
  rules otherwise. Three of them bind beyond a single slice:
  - **P11, follow-mute.** A route into a submix carries `follows_mute: bool` (route field 7). It
    may be `true` only on a route into a submix. On a route into the output, `true` refuses with
    `schema.invalid_enum` at `$.routes[<i>].follows_mute`. One coefficient function,
    `graph_compiler::route_coefficients`, built on `graph::gated_route_coefficients`, serves both
    prepared and live routes.
  - **P16, the K1 interim.** From *Render a submix strip on its summed input* (#1200) until *List
    every strip in the live-control handles and file bus effects in the browser* (#1207), effect
    live controls and observation handles attach only to effects owned by a track. A bus effect
    renders through its path without live controls during that window.
  - **P17, track-named spellings keep strip meaning.** These spellings stay unchanged, and where
    one now indexes strips, its documentation says "strip index: tracks first, then submixes":
    - the browser boot word `live_control_master_track_plus_one`;
    - the meter header's `master_track_plus_one` and `track_count`;
    - the SDK option `liveControls.masterTrackPlusOne`;
    - the record's `track_index`;
    - `HostLiveControlRequest.master_track`;
    - the codes `builtin.meter.unknown_track` and `host.observation.master_track`.

    Rust-internal names that are no wire or SDK spelling may be renamed where that is clearer.
  - The other planner decisions are recorded in DESIGN 2.3. Among them:
    - strip lowering keyed by the submix's ID (P1);
    - the route source `{ kind: "submix", submix_id, tap }` (P2);
    - route `mute` (P3);
    - route activity and signed zero (P4);
    - the indexed ramp law (P5) and ramp lengths (P6);
    - one strip-mute owner (P7);
    - strip-addressed session edits (P8);
    - strip indices (P9) and the meter frame (P10);
    - `maximum_submixes` (P12);
    - #1053 coordination (P13);
    - no performance tier committed blind (P14);
    - the transparent spec-less submix (P15).
- **Wire identity.** Every wire change above is an in-place V1 amendment, on the #1063 and decision
  12 precedent. Wire tag 2 of the route source is kept, and its tap becomes required.
  `submix_output` becomes an unknown token. Route fields 6 (`mute`) and 7 (`follows_mute`) are
  appended. The C ABI's `reserved[0]` of the compile limits becomes `maximum_submixes`. Nothing is
  renumbered, and a retired code is refused, never reallocated. There is no `ABI_VERSION` bump.
- **Owner questions (DESIGN 8.2), all open on 2026-10-02.** No filed slice depends on an answer.
  - **Q1** (may a submix override a console slot's `link_mode`?): open. The planner recommends no.
  - **Q2** (bound route values to the fader's `[-144, 24]` dB and the matrix's `[-1, 1]`?): open.
    The planner recommends yes. Deferred item O11, bounding route values and publishing their
    live-control metadata, waits on the answer, and nothing is filed for it.
  - **Q3** (accept the latency that each bus level adds through latent console slots?): open. The
    planner recommends accepting it.
  - **Q4** (the SDK defaults `followsMute` to `true` on a route into a submix): open. It is the
    planner's default, and the owner may reverse it.
  - Q5 is not asked. It arises only if a measurement shows that D9's route order blocks folding.
- **Performance.** The bus-and-send benchmark rows and their baseline are three standalone
  successor issues, outside the umbrella: *Add a bus-and-send row to the native console benchmark*
  (#1227), *Add the bus-and-send session to the browser mixing benchmark* (#1228) and *Record the
  bus-and-send baseline and its route-work profile* (#1229). They never hold #1196 open. Whether
  route fusion (deferred item O1) earns a brief is the weekly performance pass's decision, made on
  #1229's numbers (DESIGN 6.3, P14). No performance tier is committed blind.
- **#1053 and #210.** #1053's spec carries the coordination rule (P13): until the slice named there
  lands, a committed-model delta that touches a submix strip, the mute of a follow-mute source, or
  (later) a VCA member's fader is structural on the C ABI. #210's "Live send levels" bullet is owned
  by #1196: live send levels in the browser through *Ramp live send coefficients on the render
  plane* (#1220) to *Enumerate sends and drive them from the SDK* (#1223), and on the C ABI by
  *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225). The N-output
  part of #210 is unchanged.

The slices, their dependencies, their gates and the batches are in #1196.

- **Batch order.** The batches run in order, one push each (DESIGN 9.5):
  - K0: this record and #1198;
  - K1: #1199-#1205, the submix-strip schema;
  - K2: #1206-#1214, host surfaces;
  - K3: #1215-#1224, route mute, follow-mute and live sends in the browser;
  - C1 (after #1053): #1225 and #1226, the C ABI;
  - BM, after K3: #1227-#1229.
- **The VCA umbrella** is filed when K3 closes and runs after it.
- **Approved, not landed.** `AGENTS.md` states these promises with a decision-13 qualifier until
  they land, and each batch's closing slice removes its qualifiers in its own PR:
  - *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205) removes
    them from the dual-mono strip, chain, console-slot and seven-tap sentences;
  - *Let a send follow its source strip's mute live in the browser* (#1224) removes them from the
    route-mute and follow-mute sentences;
  - the VCA batch's closing slice removes the one on the VCA sentence.
