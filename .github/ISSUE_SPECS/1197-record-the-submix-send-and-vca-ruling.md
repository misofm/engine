# Record the submix, send and VCA ruling

Slice 00 of *Submix strips and live aux sends* (#1196). Batch K0, with *Iterate session strips, not tracks,
wherever strip semantics apply* (#1198).

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

The owner's answers on submix strips, aux sends and VCA groups are recorded once:

- `AGENTS.md` says what the engine has been approved to promise, and marks every promise that has
  not landed yet, so no agent assumes it;
- the bus and send umbrella, its 28 slices and the three successor performance issues exist as
  numbered specs with matching GitHub issues;
- the two open specs this work touches carry a note of what they now share with it.

This slice changes no code.

## Context (verified on `fe8ac679`)

- **Submixes and routes today.**
  - A submix is a bare summing node (`Submix { id }`, `crates/session/src/model.rs:577`).
  - Routes have no mute, and their gain is a bind-time constant (D3,
    `crates/graph/src/runtime.rs:22-24`).
- **The owner's words (2026-10-02)** are quoted verbatim in the ruling (W1-W5). On them:
  - the owner decided that buses keep the same console effects as tracks ("let's keep the same
    console effects for buses as well"), that tracks route into a submix controlled by a single
    fader, and that reverb is held for separate scoping;
  - the owner delegated two questions to the adversarially verified opinion: "VCA groups and
    submix inputs";
  - the owner gave direction the planner reads into concrete rules: no human-UX limits on agents'
    controls, and SIMD as a core philosophy;
  - the owner did **not** review planner decisions P1-P17 one by one. W5's "plan out the items
    above, adversarially verify, then implement" authorizes implementing the verified plan, not
    each planner decision.
- **The design and its reviews.**
  - The design and decision record is `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`, revision 2.
  - It was adversarially verified three times. `VERIFY-1.md` upheld both delegated decisions and
    the ramp law with amendments; `VERIFY-2.md` returned PASS-WITH-FIXES, and every fix is folded
    in; `VERIFY-3.md` returned PASS-WITH-FIXES on revision 2, and `APPLIED-3.md` applies every
    finding.
  - `REVISION-1.md` and `REVISION-2.md` map every finding to its fix.
- **Ruling format.** Owner rulings live in `docs/rulings/`.
  - Decisions 1-8 are in `engine-footprint-2026-09-28.md`, and 9-12 in `engine-footprint-2026-09-29.md`.
    Numbering continues across files, so the next is decision 13.
  - Decision 12 (`engine-footprint-2026-09-29.md:37` onward) is the format to follow:
    - a context paragraph naming the umbrella, its spec path, the verifier, the base commit and the
      verify document;
    - "The owner answered ... on `<date>`. Ruling:";
    - bold-lead bullets;
    - a closing paragraph on slices and batch order.
- **The `AGENTS.md` sentences this ruling touches** ("Approved audio architecture"):
  - `:24`: "Tracks are dual-mono: left and right have independent state and parameters. ...";
  - `:28`: "The console sections are session-level: ... every track carries every slot, in that
    order, ...";
  - `:30`: "Meters may observe any boundary without changing signal flow. Send taps are explicit
    stable enum values: ...".
- **Specs this work touches.**
  - Spec #210 (`.github/ISSUE_SPECS/210-strip-and-console-scope-chassis-intrinsic-features-solo-n-output-buses-monitor-mixes.md:17`)
    has a "Live send levels" bullet: route gains are prepared-only, and cue mixes need live send
    levels. This umbrella delivers that: slices 22-25 in the browser and slice 27 on the C ABI.
  - Spec #1053 (`.github/ISSUE_SPECS/1053-deliver-value-only-fader-mute-and-pan-transactions-to-the-running-c-abi-plan-thr.md`).
    - Its D1 (`:38-40`) classifies a committed-model delta as live when `fader` and
      `matrix_or_pan` are the only fields that differ. "Edit opcodes are not inspected." It names no
      strip kind, and it pushes only track lanes.
    - Its A2 D1 "Add:" bullet (`:145-147`) is the extension point.
    - Its Scope 2 keeps `track_controls` in capi's `ProviderEpoch` (`:23-24`).
    - Three divergences follow from this umbrella (DESIGN P13; VERIFY-1 MAJOR-3; VERIFY-2 M11):
      after batch K1 a submix has a `fader` and a `matrix_or_pan`, which #1053 would classify as live
      but never push; once routes carry `follows_mute`, a prepared route coefficient depends on its
      source strip's mute, which #1053's live path would push without the send; and once VCAs exist,
      a member's prepared fader depends on its VCAs.
    - Two host-core renames in this umbrella touch fields #1053's capi code reads:
      `HostLiveControlHandles.tracks` becomes `strips` (*List every strip in the live-control
      handles and file bus effects in the browser*, #1207), and `track_controls` becomes `strip_controls`
      (*Give every strip one mute owner and live-control producers in host-core*, #1211).
- **CI routing.** `scripts/ci-path-router.py` (`:18-19`, `EVIDENCE_PREFIXES` and `EVIDENCE_FILES`)
  classifies `docs/` and `.github/ISSUE_SPECS/` as evidence, but **not** `AGENTS.md`.
  `python3 -B scripts/ci-path-router.py --event push --path AGENTS.md --path docs/rulings/x.md`
  prints `full` (checked), so a change that includes `AGENTS.md` runs every Rust job.

## Decisions frozen for this slice

- **D1.** The ruling is **decision 13**, in a new file.
- **D2. What is filed here:** the umbrella *Submix strips and live aux sends*, its slices 01-28
  (DESIGN 9.1, titles exact; slice 18 is filed as two issues, 18a and 18b), this slice's own spec
  (filed as the umbrella's first child, so every local spec has its GitHub issue in the same
  checkpoint, `AGENTS.md`), and the three successor performance issues BM1-BM3 (DESIGN 9.2) as
  **standalone** issues, not umbrella children, each naming its dependency. The umbrella closes on
  shipped product (slices 00-28); BM1-BM3 never hold it open.
- **D3. What is not filed here:**
  - the VCA umbrella (`docs/handoffs/submix-sends-2026-10-02/issues/V0`-`V5`) is recorded in the ruling as in scope; the root files it
    when batch K3 closes (the verdict and push of *Let a send follow its source strip's mute live
    in the browser*, #1224), with its anchors re-verified then;
  - deferred item O11 (bounding route values and publishing their metadata) is recorded as
    deferred until the owner answers DESIGN question Q2; nothing is filed for it.
- **D4.** Owner questions Q1-Q4 that are still open when this lands are recorded as open. No filed
  slice depends on an answer (DESIGN 8.2).
- **D5. Decided, not landed.** Every amended `AGENTS.md` sentence that describes behaviour no code
  has yet carries a qualifier that names its authority and the slice it lands with:
  - an owner decision or an owner-delegated answer: "Approved by decision 13 (#1196) as owner
    decision O1, landing with *<slice title>* (#N): ...";
  - owner direction read by the planner, or a planner decision: "Planned under decision 13 (#1196)
    as planner decision P3, subject to owner review, landing with *<slice title>* (#N): ...".

  A qualifier never calls a planner decision "approved". Each batch's closing slice removes, in its
  own PR, the qualifiers of the behaviours its batch landed:
  - *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205) for the
    dual-mono strip, chain, console-slot, strip-insert and seven-tap sentences;
  - *Let a send follow its source strip's mute live in the browser* (#1224) for the route-mute and
    follow-mute sentences;
  - V4, *Enumerate VCA groups and drive them from the SDK*, which closes the VCA batch V1-V4, for
    the VCA sentence (its draft carries the deliverable).

  This slice only writes them.

## Deliverables

1. **Ruling record.** Write `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md` in the decision-12
   format: H1, a one-line preface, then `## Decision 13: Submix strips, live aux sends and VCA groups
   (#1196)`. It records:
   - the owner's words of 2026-10-02, verbatim, and for each of O1-O5 (DESIGN 2.1) whether it is
     an owner decision, owner direction read by the planner, or a delegation;
   - delegated decisions (a) and (b) (DESIGN 2.2), with VCA sequenced as its own umbrella that
     starts once batch K3 closes, and only its C ABI slice waiting on #1053;
   - planner decisions P1-P17 (DESIGN 2.3), recorded as subject to owner review because the owner
     has not reviewed them one by one, in particular:
     - P11: `follows_mute` may be `true` only on a route into a submix; `true` on a route into the
       output refuses with `schema.invalid_enum` at `$.routes[<i>].follows_mute`;
     - P16: from *Render a submix strip on its summed input* (#1200) until *List every strip in the
       live-control handles and file bus effects in the browser*, effect live controls and
       observation handles attach only to track-owned effects;
     - P17: the track-named spellings that keep strip meaning;
   - the owner's answers to Q1-Q4, or "open" for each one still open;
   - that the benchmark rows and the baseline are successor issues BM1-BM3, and that the route
     fusion decision belongs to the weekly performance pass (DESIGN 6.3, P14);
   - a closing paragraph on the batches (DESIGN 9.5) and, for each D5 qualifier, the slice whose
     landing removes it.
2. **`AGENTS.md` amendments**, in the same commit as the ruling, in "Approved audio architecture",
   each phrased per D5 where the behaviour has not landed, with the authority the ruling gives it:
   - `:24`: "Tracks are dual-mono" becomes "Strips (tracks and submixes) are dual-mono" (the
     submix half lands with *Render a submix strip on its summed input*).
   - After the chain line: the chain applies to every strip, and a submix's input is the sum of the
     routes that target it (lands with *Render a submix strip on its summed input*). The binding
     text does not pin a summation order.
   - `:28`: "every track carries every slot, in that order" becomes "every strip carries every slot,
     in that order" (lands with *Carry every console slot on every submix strip*, #1202), and
     "Inserts are per track ... a track's inserts" becomes "Inserts are per strip ... a strip's
     inserts" (lands with *Render a submix strip on its summed input*).
   - `:30`, the send-tap sentence: the seven taps exist on every strip (lands with *Tap a submix
     strip at any of the seven send points*, #1203); a route may be muted (lands with *Mute a route in the
     session*, #1216), and a route into a submix may follow its source strip's mute (lands with *Let a route
     into a submix follow its source strip's mute in the session*, #1218).
   - One new sentence: a VCA group is control-only; it carries no audio, its dB offset adds to its
     members' faders, and its mute ORs into their mutes (lands with the VCA umbrella).
   - Nothing about performance tiers, which are deferred.
3. **Umbrella spec.** Write `.github/ISSUE_SPECS/1196-submix-strips-and-live-aux-sends.md` from
   `DESIGN.md`, with the owner's answers folded in. It lists slices 01-28 (18a and 18b included) with their GitHub numbers
   and names BM1-BM3 as successors outside the umbrella.
4. **Slice specs.** One spec per slice 01-28 of this umbrella (18a and 18b each get one), and this
   slice's own spec, each renumbered to its GitHub number. Each
   dependency line names the exact published title (DESIGN 9.1).
5. **Successor specs.** `BM1`, `BM2` and `BM3` (DESIGN 9.2), renumbered to their GitHub numbers,
   each a standalone issue whose dependency line names *Let a send follow its source strip's mute
   live in the browser* (BM1), BM1's title (BM2) or BM2's title (BM3).
6. **GitHub issues.** One per filed spec, created in the same checkpoint. Confirm that every number
   and title matches its local filename (`AGENTS.md`, delivery-control rules).
7. **Annotate #210.** After the "Live send levels" bullet (`:17`), append: owned by *Submix strips
   and live aux sends*: slices 22-25 deliver live send levels in the browser and slice 27 on the C
   ABI. The N-output part of #210 is unchanged.
8. **Annotate #1053.** Append to its A2 D1 "Add:" bullet (`:145-147`), phrased on the committed
   model because D1 inspects no opcodes:

   > Coordination with *Submix strips and live aux sends* (decision 13). Until the named slice
   > lands, these committed-model deltas are **structural**: any change to a field of a **submix**
   > strip, until *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225); a
   > change to `left_mute` or `right_mute` of a strip that, in the post-commit model, is the source
   > of a route with `follows_mute: true`, until *Let C ABI sends follow their source strip's mute
   > live*; and, once the VCA umbrella lands, any fader field of a VCA member, until that umbrella's
   > C ABI slice. Whichever of #1053 and the slice that creates the dependency lands second
   > implements the rule in `live_builtin_delta`: *Declare the submix strip in the session grammar
   > and wire* for submix-strip fields, *Let a route into a submix follow its source strip's mute
   > in the session* for follow sources, and the VCA preparation slice for VCA members. Two host-core
   > fields #1053's capi code reads are renamed by that umbrella: `HostLiveControlHandles.tracks`
   > becomes `strips` (*List every strip in the live-control handles and file bus effects in the
   > browser*) and `track_controls` becomes `strip_controls` (*Give every strip one mute owner and
   > live-control producers in host-core*); whichever of #1053 and a renaming slice lands second
   > updates the other's uses. If #1053's C ABI preparation requests live controls with a queue
   > depth after *Produce live send records from host-core* (#1221) lands, every C ABI plan gets route
   > lanes: keep that request builtins-only until *Deliver value-only send and submix-strip edits to
   > the running C ABI plan*, or re-pin the capi resource oracles and `audit capi` with a reason.
9. **Index.** Add one line to `docs/IMPLEMENTATION_PLAN.md`'s "Index" section (`:39` onward) for the
   umbrella.

## Authorized paths

- `docs/rulings/` (one new file)
- `AGENTS.md`
- `.github/ISSUE_SPECS/`:
  - new files (the umbrella, this slice, slices 01-28 with 18a and 18b, BM1-BM3), including
    renaming a new file so its name is the slug of its GitHub title;
  - in *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205), the
    qualifier-removal deliverable and its authorized paths;
  - the annotations to the `210-*.md` and `1053-*.md` specs.
- `docs/IMPLEMENTATION_PLAN.md` (one index line)
- `docs/handoffs/submix-sends-2026-10-02/` (new folder): the design record (`DESIGN.md`), the
  verifications (`VERIFY-1.md` to `VERIFY-3.md`), the revisions (`REVISION-1.md`, `REVISION-2.md`,
  `APPLIED-3.md`), `ISSUE-MAP.md` and the unfiled VCA drafts `issues/V0`-`V5`
- this spec

## Non-goals

- No code, fixture or schema change.
- No change to decision 12's text, except where `AGENTS.md` paraphrases it.
- No VCA spec filed, and nothing filed for O11.

## Hazards

- **Paraphrase drift.** `AGENTS.md` is binding on every agent. Each amended sentence must say only
  what the ruling says: strips, taps, mute, follow-mute into a submix, and VCA as control-only.
  Each qualifier claims exactly the authority the ruling gives that point. An amended sentence
  must not contradict an unamended one; "Inserts are per track" next to "the chain applies to every
  strip" did, in attempt 1.
- **Promising before landing.** An unqualified sentence about submix strips would tell an agent a
  feature exists that K1 has not shipped. D5's qualifier is mandatory on every not-yet-landed
  sentence.
- **CI cost.** Because `AGENTS.md` routes `full`, the K0 push runs the Rust jobs once. That is
  expected; it is not a reason to split `AGENTS.md` out of K0.

## Objective gates

1. `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh` pass.
2. `.github/ISSUE_SPECS/` and `gh issue list --state open` agree on number and title for every new
   spec. The umbrella lists every slice with its GitHub number; BM1-BM3 are open and are not
   listed as umbrella children.
3. `python3 -B scripts/ci-path-router.py --event push --path AGENTS.md --path docs/rulings/x.md`
   prints `full`. That is expected, because `AGENTS.md` is not an evidence path, and the K0 push
   runs the Rust jobs once.
4. No test is added. The slice changes no behaviour.

## Evidence

- Links to the created GitHub issues.
- The owner's verbatim answers, or "open", for Q1-Q4.
- The diff of the two annotated specs and of `AGENTS.md`.

## Decision record

- **Attempt 1** (`f622dad1`) failed review with two MAJORs: `AGENTS.md` called planner decisions
  P3 and P11 "approved", and labelled all of decision 13 an owner decision; and "Inserts are per
  track" contradicted "the chain applies to every strip".
- **Attempt 2** learned what the owner actually said and recorded it verbatim (W1-W5). O1 (the
  console on buses), O2 (reverb held) and the delegation (VCA groups, "submix inputs") are the
  owner's; the concrete reading of "no human-UX limits" (O4) and how SIMD applies (O5) are owner
  direction read by the planner; P1-P17 are the planner's, subject to owner review. D5's template
  now distinguishes "Approved by decision 13" from "Planned under decision 13 ..., subject to owner
  review". The insert sentence became per-strip with its own qualifier, and #1205 removes it and
  makes "track-locally" "strip-locally" at K1. The VCA qualifier's removal belongs to V4, whose
  draft carries it. Five spec files were renamed to the slug of their GitHub titles.

## Dependencies

None. The owner's words of 2026-10-02 (W5) meet the design dependency: the owner decided O1 and O2,
delegated VCA groups and submix inputs to the adversarially verified opinion, and asked for the
plan to be verified and then implemented. The owner did not accept `DESIGN.md` revision 2 as a
whole, so the ruling records P1-P17 as subject to owner review. Open owner questions do not block
it (D4).

## Standing rules for the implementer

- Docs and specs only. Do not touch code paths.
- Keep the in-place V1 identity rule: any wire code, field ID or spelling named in the ruling is
  appended or retired, never renumbered or reallocated.
- Do not rewrite or discard another agent's history in the annotated specs. Append only.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
