# Submix strips and live aux sends

The owner's decisions of 2026-10-02 and the planner's verified design are recorded as **decision
13** (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md`), which quotes the owner's words and
marks each point's authority. A mixing console has buses as well as
channels: a drum bus with its own EQ, compressor and fader, and a delay return fed by sends.
Today the engine has neither. This umbrella delivers both: submix strips and live aux sends.

**Provenance.**

- The planner's design and decision record is `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`
  (revision 2, written against `fe8ac679`). It is the design of record for this umbrella, and every
  section number below (for example "DESIGN 5.7") refers to it.
- Three Sol adversarial reviews by verifiers who wrote none of the plan, all in the same folder:
  - `VERIFY-1.md`: FAIL as written, but both delegated decisions and the ramp law upheld with
    amendments;
  - `VERIFY-2.md`: PASS-WITH-FIXES on revision 1, folded into revision 2 (`REVISION-1.md`,
    `REVISION-2.md`);
  - `VERIFY-3.md`: PASS-WITH-FIXES on revision 2, applied in `APPLIED-3.md`.
- **Authority.**
  - **The owner decided:** buses keep the same console effects as tracks, each submix with its own
    parameters and bypass per slot, and tracks route into a submix whose single fader controls them
    (O1). Reverb is out of scope and is held for separate scoping (O2).
  - **The owner directed, and the planner reads the direction into rules subject to owner review:**
    a submix is a full strip, so it also has inserts, mute and pan/matrix (O1); agents' controls
    have no human-UX limits, read as arbitrary bus counts, nesting, any tap, a full 2x2 per send and
    fan-out (O4); SIMD is a core philosophy, applied as DESIGN section 6 says (O5).
  - **The owner delegated** two questions, "VCA groups and submix inputs", and deferred to the
    adversarially verified opinion. The planner reads "submix inputs" as the submix strip's input
    section and, with it, its channel model. Those opinions (VCA in scope as its own umbrella; the
    submix strip is the track's dual-mono strip with all five input-section keys) carry the owner's
    delegated authority.
  - **The planner decided** P1-P17 (DESIGN 2.3). Adversarial verification upheld them, and they
    are subject to owner review: the owner has not reviewed them one by one. Owner questions Q1-Q4
    are open (see below).

## Problem

- **A submix is a bare summing node** (`Submix { id }`, `crates/session/src/model.rs:577`). It has
  no input section, no console slots, no inserts, no fader, no mute and no pan, and a route can
  leave it only from its output.
- **Routes have no mute.** Their gain is a bind-time constant (`crates/graph/src/runtime.rs:22-24`),
  so a send level can change only through a plan replacement.
- **A muted track still feeds its pre-fader sends.** The probe in DESIGN 3.2 measured 0.3155 and
  0.3549 where 0 was expected.

## Design (decided; DESIGN section 5)

- **A submix is a strip.** It carries exactly what a track carries after its source:
  - the input section (`polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz`, `delay_samples` per lane);
  - every session console slot, each with its own `{slot, bypass, params}`;
  - ordered inserts;
  - a per-lane fader and mute;
  - `pan` or `matrix`.

  Its input is the master-plan D9 sum of the routes that target it. It lowers to the track's stage
  nodes and racks, keyed by the submix's ID, so its console slots bank with the same programs at
  their own dependency level, padded, exactly as decision 12 requires. A nonzero `delay_samples`
  delays the summed input. A submix strip is dual-mono and never takes the mono collapse.
- **Sends are routes.** An aux send is a route from any of a strip's seven taps to a submix input.
  Fan-out is several routes, and nesting is a route from one submix's tap to another's input. No
  count is compiled anywhere, and acyclicity is the graph compiler's existing check.
- **Route mute and follow-mute.** Every route gains `mute`. A route into a submix may set
  `follows_mute`; on a route into the output it refuses. One coefficient function serves prepared
  and live routes. A muted or follow-muted undelayed route is inactive, so it is neither mixed nor
  loaded. A delayed route is never inactive.
- **Live sends.** In a plan prepared with live controls, a route into a submix has live gain, mute
  and matrix. They ramp on the render plane with the indexed ramp law (DESIGN 5.7). Adding, removing
  or re-pointing a route, and any edit to a route into the output, stay plan replacements.
- **One strip-mute owner.** Strip mute and solo compose in one place, sized per strip (tracks, then
  submixes); submix entries are solo-safe.
- **SIMD.** The correctness slices keep today's route ops and reductions, so nothing is optimised
  blind. The only new kernel, the live ramp, is vectorised over frames. One implementation shape
  serves every target, and only the lane width differs (DESIGN section 6).

Wire changes are in-place V1 amendments: nothing is renumbered, a retired code is refused and never
reallocated, and there is no `ABI_VERSION` bump (DESIGN 5.11).

## Slices

Each slice is its own issue, and each issue's spec is in `.github/ISSUE_SPECS/` under its number.
Dependency lines in the slices name the exact published titles.

| # | Issue | Title | Depends on |
|---|---|---|---|
| 00 | #1197 | Record the submix, send and VCA ruling | the owner's answers of 2026-10-02 (met) |
| 01 | #1198 | Iterate session strips, not tracks, wherever strip semantics apply | #1197 |
| 02 | #1199 | Declare the submix strip in the session grammar and wire | #1198 |
| 03 | #1200 | Render a submix strip on its summed input | #1199 |
| 04 | #1201 | Delay a submix strip's summed input | #1200 |
| 05 | #1202 | Carry every console slot on every submix strip | #1201 |
| 06 | #1203 | Tap a submix strip at any of the seven send points | #1202 |
| 07 | #1204 | Address submix strips in session edits | #1203 |
| 08 | #1205 | Build submix strips and bus taps in the SDK and teach agents to author them | #1204 |
| 09 | #1206 | Count and cap submix strips in host preparation and the C ABI | #1205 (K1 pushed) |
| 10 | #1207 | List every strip in the live-control handles and file bus effects in the browser | #1206 |
| 11 | #1208 | Meter any boundary of a submix strip and designate a master strip in host-core | #1207 |
| 12 | #1209 | Carry submix strips in the browser meter frame | #1208 |
| 13 | #1210 | Name submix strips in the browser session map and the SDK measurement | #1209 |
| 14 | #1211 | Give every strip one mute owner and live-control producers in host-core | #1207 |
| 15 | #1212 | Add the notSoloable command reason to every vocabulary spelling | #1205 (K1 pushed) |
| 16 | #1213 | Address submix strips in browser live commands | #1209, #1211, #1212 |
| 17 | #1214 | Drive submix strips from the SDK live controls | #1210, #1213 |
| 18a | #1215 | Gate every route's coefficients through one function | #1205 (K1 pushed); merges after #1214 |
| 18b | #1216 | Mute a route in the session | #1215 |
| 19 | #1217 | Skip an inactive route in its destination's sum | #1216 |
| 20 | #1218 | Let a route into a submix follow its source strip's mute in the session | #1217 |
| 21 | #1219 | Ramp a send's coefficients with the indexed ramp kernel | #1205 (K1 pushed); may be built beside #1215-#1218 |
| 22 | #1220 | Ramp live send coefficients on the render plane | #1218, #1219 |
| 23 | #1221 | Produce live send records from host-core | #1220, #1211 |
| 24 | #1222 | Admit live send commands in the browser | #1221, #1214 |
| 25 | #1223 | Enumerate sends and drive them from the SDK | #1222 |
| 26 | #1224 | Let a send follow its source strip's mute live in the browser | #1223 |
| 27 | #1225 | Deliver value-only send and submix-strip edits to the running C ABI plan | #1224, #1053 |
| 28 | #1226 | Let C ABI sends follow their source strip's mute live | #1225 |

**This umbrella closes on shipped product**, when slices 00-28 (18a and 18b included) have closed.
Slices 27 and 28 wait on #1053.

### Successors outside the umbrella

Three standalone performance issues measure what shipped. They are tooling and evidence, not
launch-critical features, and they never hold this umbrella open:

- #1227 *Add a bus-and-send row to the native console benchmark*, after #1224 (K3 pushed);
- #1228 *Add the bus-and-send session to the browser mixing benchmark*, after #1227;
- #1229 *Record the bus-and-send baseline and its route-work profile*, after #1228.

Whether route fusion (deferred item O1) earns a brief is the weekly performance pass's decision on
#1229's numbers (DESIGN 6.3, P14).

### VCA groups (a separate umbrella, not yet filed)

VCA groups are in scope (delegated decision (a)). Their umbrella and slices are drafted at
`docs/handoffs/submix-sends-2026-10-02/issues/V0`-`V5`. The root files them when batch K3 closes
(#1224's Sol PASS and the K3 push), and re-verifies their anchors then. Nothing in this umbrella
depends on VCA.

## Batches (CI-conscious mode: one push per batch)

| Batch | Issues, in merge order | Why this boundary |
|---|---|---|
| K0 | #1197, #1198 | The ruling and a class-A refactor. `AGENTS.md` routes CI `full`, so K0 runs the Rust jobs once. |
| K1 | #1199-#1205 | The submix-strip schema change, pushed once. The SDK is out of step from #1199 to #1205 (the #1084 C3 precedent). Live-controlled boots keep working through P16. |
| K2 | #1206-#1214 | Host surfaces for submix strips. The browser artifact changes once. |
| K3 | #1215-#1224 | Route mute and follow-mute (two migrations, one push), the activity rule, live sends, browser sends, live follow-mute. |
| C1 (after #1053) | #1225, #1226 | The C ABI value-only path, then its follow-mute composition. |
| BM (after K3) | #1227, #1228, #1229 | Successor tooling and evidence; not launch-critical. |
| VCA (after K3) | V1-V4, then V5 after #1226 and #1053 | Filed when K3 closes. |

`AGENTS.md` allows one launch-critical implementation at a time, and K2 and K3 touch overlapping
host-web and host-core files, so the batches run in order. K3's #1215-#1220 may be drafted during K2
review, but are implemented on the K2 head. C1 and the VCA batch both touch host-core, so the root
never runs both at once.

**First product slice.** The smallest audible product is K1: a drum bus with console EQ and
compressor, a glue-compressor insert and a fader, and a delay return fed by sends, authored from the
SDK and rendered by the browser and the C ABI.

## Coordination

- **#1053** (P13). Until the C ABI composes the dependants, a committed-model delta that changes a
  submix strip's field (until #1225), the mute of a follow-mute source (until #1226), or (later) a
  VCA member's fader is structural. #1053's spec carries the rule and the two host-core renames that
  touch its code.
- **#210.** Its "Live send levels" bullet is delivered here: #1220-#1223 in the browser and #1225 on
  the C ABI. Its N-output part is unchanged.
- **#1054.** Route ramp lengths use each change's own smoothing until #1054 gives the session a
  smoothing table (P6). On the C ABI, live records use #1053's fixed ramp.

## Owner questions (DESIGN 8.2), open on 2026-10-02

No filed slice depends on an answer.

- **Q1.** May a submix override a console slot's `link_mode`? Recommendation: no. Use a linked
  insert for bus glue, and bypass the console compressor on buses.
- **Q2.** Should route values be bounded to the fader's `[-144, 24]` dB and the matrix's `[-1, 1]`?
  Recommendation: yes. Deferred item O11 waits on the answer.
- **Q3.** Is the latency that each bus level adds through latent console slots accepted?
  Recommendation: accept.
- **Q4.** The SDK defaults `followsMute` to `true` on a route into a submix. Say if you want the
  default reversed.

## Gates every slice inherits

DESIGN section 7 lists the commands:

- format, lint and docs;
- workspace and per-crate policy;
- the workspace, DSP and tools tests;
- 4-lane coverage;
- wire parity;
- render artifacts;
- the browser build and gates;
- the SDK gates;
- the C ABI gates.

It also lists the rules for comparing a live edit against a fresh plan, and for the signals that
live-route gates feed. Every new or rewritten test carries its one-sentence test-value answer in its
issue. Digests and canonical graph text are re-pinned only where the issue lists them, each with its
reason.

## Deferred (designed, not filed; DESIGN section 10)

O1-O11 are designed but not filed, among them:

- route fusion (O1);
- silence skips (O2);
- the epilogue folds (O3, O4);
- bus bank alignment (O5);
- live output routes (O9);
- route value bounds and their metadata, after Q2 (O11).

Each item names its trigger in DESIGN section 10. Further follow-ups, listed in DESIGN 8.2, are not
designed yet: `follows_pan`, bus solo and implied upstream solo, and VCA trim of sends.

Optional successor, not designed (the #1198 verdict, MINOR-2): a routed-sidechain case in the graph
fixture corpus, which owns the sealed text, so a change to the `.sidechain` edge-path suffix turns a
gate red. No test pins that suffix today.

Known gap, not filed (found by #1205's implementer; the #1205 verdict, MINOR-2): the engine
accepts an automation target whose `entity_id` names a submix (#1199 D6; a `console` target
addresses the submix's entry, #1202), but the SDK's `.automation()` builder still resolves
`trackId` against tracks only, and `enginectl`'s target is `trackId` only, so neither can author
it. Stored automation renders nothing yet (#1058), so the impact is nil today. The
author-session skill says so and points agents at the JSON. A successor adds `{ submixId }` beside
`trackId` in the builder and in `enginectl`.
