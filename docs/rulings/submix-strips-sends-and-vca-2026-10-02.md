# Submix strips, sends and VCA groups: owner rulings and planner decisions of 2026-10-02

Follows decision 12 (`engine-footprint-2026-09-29.md`), recorded from the owner's words on
2026-10-02, quoted below. Each bullet names whose authority it carries.

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

**The owner's words** (2026-10-02, quoted verbatim, typos included). The owner's original requests:

> W1. "Submixes: we should be able to set the output of a track to an input of a submix track. For
> example if I have drum tracks, I should be able to route them all to a drum submix so I can
> control the entire drum kid with a single fader. Please draw experience from how submixes work on
> traditional consoles as well as modern DAWs, and then let's map out the most efficient and
> performant design for our engine."
>
> W2. "Sends: we should be able to configure aux sends. The most obvious use case is for global
> effects like reverb. Please draw experience from both traditional consoles and modern DAWs, and
> let's get the best design implemented."
>
> W3. "miso engine is primarily an agent-driven engine in terms of configuration. So don't limit
> the UX or scope of the engine controls with human UX in mind."
>
> W4. "one of our core design phiosophies is maximizng SIMD usage. Please think about how SIMD can
> maximize performance of submixes and sends."

The owner's answer, after being told that large-format desks give buses the same EQ and dynamics
as channels, and that each track has its own settings for each console slot:

> W5. "Ok let's keep the same console effects for buses as well. For VCA groups and submix inputs,
> I'll defer to your adversarially verified opinions. Reverb-wise, let's hold off on that so we can
> sepnd more time scoing that. I have to sleep now. Please plan out the items above, adversarially
> verify, then implement - use a fresh agent for each step, opus 5.5 please."

W5's "submix inputs" answers the last of the four questions in the planner's first reply, verbatim:

> "**Submix input:** a bus sums panned tracks, so I'd make it a true L/R stereo strip and not
> dual-mono. Agreed?"

**Authority.** Every point below carries one of four kinds:

- **owner decision**: the owner's words decide it directly;
- **owner direction, read by the planner**: the owner gave a directive, and the concrete rule is
  the planner's reading of it, subject to owner review;
- **owner-delegated**: the owner deferred the question to the adversarially verified opinion (W5);
- **planner decision**: the planner's alone, upheld by adversarial verification and subject to
  owner review.

DESIGN 2.1 labels O1-O5 "owner decisions". This record is narrower, and where the two differ, this
record governs. W5's "Please plan out the items above, adversarially verify, then implement" is
the owner's authority for the slices to implement the verified plan, planner decisions included,
until the owner rules otherwise. It is not the owner's review of each planner decision.

Ruling:

- **O1, owner decision: buses keep the same console effects** (W5). Every submix carries every
  session console slot, with its own `{slot, bypass, params}` per slot, as each track does. Decision
  12 applies to a submix exactly as to a track: its console slots always bank, padded; a bypassed
  lane stays in its bank; latency is always paid; banking couples cost, never bits. The owner also
  decided, in W1, that a track's output can be routed to a submix's input, and that a submix has a
  fader that controls everything routed into it.
  - **Owner direction, read by the planner** (W1's "submix track"): a submix is a full strip, so it
    also carries ordered inserts, a per-lane mute and `pan` or `matrix`, as DESIGN 2.1's O1 row
    lists. Subject to owner review.
  - **Design consequence, not the owner's words:** a submix's input is the sum of the routes that
    target it, reduced as every reduction is (DESIGN 3.1 D9: stable edge-ID order).
- **O2, owner decision: reverb is out of scope** (W5: "let's hold off on that"). It is held for
  separate scoping. The planner's consequence: `miso.delay` is the send effect in tests. It declares
  zero latency, so PDC fixtures use the true-peak limiter.
- **O3, owner delegation** (W5: "For VCA groups and submix inputs, I'll defer to your adversarially
  verified opinions"). The answers are (a) and (b) below. W5's "submix inputs" answers the planner's
  question 4, quoted after W5, so the submix strip's **channel model** is the directly delegated
  part. The planner also reads "submix inputs" as covering the strip's **input section**, which
  DESIGN 2.2b answers with the channel model as one question. The input-section half rests on that
  reading.
  - **Subject to owner review:** (b) keeps trim and polarity, which the planner had told the owner
    a bus would drop. The planner's second reply, before W5, said that a submix "drops the
    source-specific parts of the input section, such as trim and polarity". The owner has not been
    told of this reversal.
  - (b)'s dual-mono strip is the verified opinion, and it reverses question 4's own lean toward "a
    true L/R stereo strip and not dual-mono". The owner deferred to the verified opinion, not to
    the lean.
- **O4, owner direction, read by the planner** (W2 and W3). Sends are for agents, so the engine sets
  no human-UX limits. The planner reads that as: arbitrary bus counts, nesting, any tap, a full 2x2
  per send, and fan-out. These are subject to owner review. Two limits are standing `AGENTS.md`
  rules, not new ones: the graph stays acyclic (feedback is a future capability), and only
  configured resources bound it, never a compiled maximum.
- **O5, owner direction** (W4, and `AGENTS.md`'s SIMD principle). SIMD is a core philosophy. How it
  applies to buses and sends (DESIGN section 6) is the planner's, subject to owner review.
- **Owner-delegated answers** (O3). VERIFY-1 upheld both opinions, and its amendments are folded
  in. The two answers below therefore carry the owner's delegated authority, (b) under the reading
  of "submix inputs" recorded at O3, with (b)'s trim and polarity subject to owner review (O3).
  - **(a) VCA groups are in scope** (DESIGN 2.2a).
    - A VCA is a control-only group. It carries no audio. Its per-lane dB offset adds to each
      member's own fader, and its mute ORs into each member's effective mute.
    - It is its own umbrella, *VCA groups* (#1239), filed on 2026-10-03 once batch K3 of #1196 was
      delivered (*Let a send follow its source strip's mute live in the browser*, #1224). Its anchors
      were re-verified on the K3 head `8c6268967`, and a fresh Sol adversarial verification was
      folded in. Its slices are #1240-#1247. The drafts (`issues/V0`-`V5` in the handoff folder) were
      split into eight slices and removed at filing; git history keeps them.
    - Only its C ABI slice, *Deliver value-only VCA edits to the running C ABI plan* (#1247), also
      waits on #1053, #1225 and *Let C ABI sends follow their source strip's mute live* (#1226).
      Nothing in #1196 depends on VCA.
    - **Planner decision at filing, subject to owner review:** P13's VCA guard is widened. It was
      "any fader field of a VCA member"; it is now "every C ABI delta while the pre- or post-commit
      model declares a VCA", until #1247. #1225's route records and #1226's follow mirror read raw
      committed mutes, so the narrow guard would let a live send edit reopen a VCA-muted member's
      following send. The specs of #1053, #1225 and #1226 carry the widened rule.
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
  owner has not reviewed them one by one. Under W5's "plan out ..., adversarially verify, then
  implement", the slices implement them as written until the owner rules otherwise. Three of them
  bind beyond a single slice:
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
    - the route source `{ kind: "submix", submix_id, tap }` (P2), the planner's reading of O4's
      "any tap" for a submix;
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
- **Owner questions (DESIGN 8.2), all open on 2026-10-02; Q1-Q4 answered on 2026-10-03.** No
  filed slice of #1196 depends on an answer. The owner's words are quoted verbatim, from the
  owner's message of 2026-10-03. Each answer names its authority kind, as above.
  - **Q1, owner direction, read by the planner** (may a submix override a console slot's
    `link_mode`?): **answered.** The owner: "We should follow whatever modern day DAWs look like."
    That is the owner's direction. The concrete rule is the planner's reading of it, subject to
    owner review: a DAW chooses stereo-linked or multi-mono per plugin instance, so a strip may
    override its console slot's `link_mode`. This reverses the planner's "no". The kernel's link
    is already a per-lane mask (DESIGN 8.2 Q1), so a per-strip override would not need to split a
    bank once the link mode leaves `EffectProgramKey`, where it is today (#1236's L1). It is filed
    as its own issue, *Let a strip override a console slot's link mode* (#1236), outside #1196.
    Until it lands, the bus-compression hazard guidance (DESIGN 2.2b) stands.
  - **Q2, owner decision (the agreed bounds)** (bound route values to the fader's `[-144, 24]` dB
    and the matrix's `[-1, 1]`?): **answered, agreed.** The owner: "This makes sense to add
    bounded min/max for these values right?" -- agreed on 2026-10-03: route `gain_db` in
    `[-144, 24]` dB, matrix coefficients in `[-1, 1]`, and a subnormal folded coefficient
    (`gain * coefficient`, for example -120 dB times `1.2e-38`, which `route_coefficients` accepts
    today, #1215 verdict INFO) flushed to `+0.0`. Deferred item O11 (DESIGN section 10) is no
    longer deferred: it is filed as *Bound route gain and matrix values* (#1237), outside #1196.
  - **Q3, owner decision (accepted as is)** (accept the latency that each bus level adds through
    latent console slots?): **answered, accepted as is.** The owner: "There's no way around this
    right?" The planner explained that every strip carries every console slot (O1) and a bypassed
    slot keeps its latency (decision 12), so a latent console slot adds its latency at every bus
    level, and that the way to avoid it is to put latent effects (a limiter) in inserts, not in the
    console. The owner accepted that. Nothing changes and nothing is filed.
  - **Q4, planner decision (the default), explained to the owner** (the SDK defaults `followsMute`
    to `true` on a route into a submix): **answered, default kept.** The owner: "Please explain
    this to me." The planner explained that a send following its source strip's mute goes quiet
    when the strip is muted or soloed out, pre-fader sends included, that only routes into a
    submix may follow, and that a monitor-style send opts out with `followsMute: false`. The owner
    asked for the explanation and raised no objection, but did not decide: the default stays
    `true` as the planner's (P11), subject to owner review.
  - **Q5** (asked on 2026-10-03: should a submix strip keep the input section's `trim_db` and
    `polarity_invert`?): **open, pending the owner's reply.** The planner explained that DAWs
    split on it (Logic and Pro Tools give an aux or bus no trim or polarity; Cubase and Studio One
    do) and recommended keeping both, as shipped by K1 (DESIGN 2.2, "Input section: all five
    keys"). Nothing changes until the owner answers.
  - The question DESIGN 8.2 held back as its own "Q5" (a class-B change of bus-sum order, only if
    a measurement shows D9's route order blocks folding) is still not asked; it is unrelated to the
    owner's Q5 above.
- **Performance.** The bus-and-send benchmark rows and their baseline are three standalone
  successor issues, outside the umbrella: *Add a bus-and-send row to the native console benchmark*
  (#1227), *Add the bus-and-send session to the browser mixing benchmark* (#1228) and *Record the
  bus-and-send baseline and its route-work profile* (#1229). They never hold #1196 open. Whether
  route fusion (deferred item O1) earns a brief is the weekly performance pass's decision, made on
  #1229's numbers (DESIGN 6.3, P14). No performance tier is committed blind.
- **#1053 and #210.** #1053's spec carries the coordination rule (P13): until the slice named there
  lands, a committed-model delta that touches a submix strip, the mute of a follow-mute source, or
  (from #1242, until #1247) any delta of a session that declares a VCA is structural on the C ABI.
  #210's "Live send levels" bullet is owned by #1196: live send levels in the browser through
  *Ramp live send coefficients on the render plane* (#1220) to *Enumerate sends and drive them from
  the SDK* (#1223), and on the C ABI by *Deliver value-only send and submix-strip edits to the
  running C ABI plan* (#1225). The N-output part of #210 is unchanged.

The slices, their dependencies, their gates and the batches are in #1196.

- **Batch order.** The batches run in order, one push each (DESIGN 9.5):
  - K0: this record and #1198;
  - K1: #1199-#1205, the submix-strip schema;
  - K2: #1206-#1214, host surfaces;
  - K3: #1215-#1224, route mute, follow-mute and live sends in the browser;
  - C1 (after #1053): #1225 and #1226, the C ABI;
  - BM, after K3: #1227-#1229.
- **The VCA umbrella** (#1239) was filed when K3 was delivered and runs after it: batch VCA is
  #1240-#1246, pushed once; #1247 follows #1053, #1225 and #1226.
- **Decided, not landed.** `AGENTS.md` states these promises with a decision-13 qualifier until
  they land. The qualifier names the point's authority: "Approved by decision 13" for an owner
  decision or an owner-delegated answer, and "Planned under decision 13 ..., subject to owner
  review" for owner direction read by the planner and for planner decisions. Each batch's closing
  slice removes its qualifiers in its own PR:
  - *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205) removes
    them from the dual-mono strip, chain, console-slot, strip-insert and seven-tap sentences, and
    makes "track-locally" "strip-locally" in the insert sentence of "Effects and plugins", and turns
    the banking paragraph's "tracks" into "strips";
  - *Let a send follow its source strip's mute live in the browser* (#1224) removes them from the
    route-mute and follow-mute sentences;
  - *Enumerate VCA groups and drive them from the SDK* (#1246), which closes the VCA batch
    #1240-#1246, removes the one on the VCA sentence. #1247 (the C ABI) is not needed for that: from
    *Apply VCA offsets and mutes at preparation* (#1242) on, VCAs apply at preparation on every host.
