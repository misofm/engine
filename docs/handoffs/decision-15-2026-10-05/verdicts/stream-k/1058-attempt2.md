FAIL

# #1058 attempt 2: adversarial verdict (stream K, stored automation research)

Reviewed: `50fb23b5f` on `codex/d15-stream-k` (attempt 1 `436cc137d`; `45c5a1819` between them is
the #1057 note, used for cross-reference only). I read the files with `git show` and from a
`git archive` copy. I did not touch the worktree. Scope: `docs/handoffs/stored-automation-1058/README.md`,
the 41 drafts under `proposed-specs/`, the #1058 spec, and
`git diff 436cc137d 50fb23b5f` of those paths. Gate 3 is judged as the coordinator reworded it: each
slice has a complete, stateless, fileable draft that lists its dependencies.

Attempt 2 fixes the attempt-1 major well. No browser placeholder, no browser-only reason and no
browser admission edit remains, and the permanent `model_only` rule is correct. But the same class
of defect is back in one draft. The attempt changed 06a so that it depends on #1293, so draft 06b
now lands after *Move browser source submission and seeks into the Worker* (#1387). Draft 06b is
still written for the worklet routing that #1387 removes in Worker mode (MA1). Because of that
MAJOR, the attempt fails. The fix is local to one draft.

## BLOCKER

None.

## MAJOR

**MA1. Draft 06b freezes a worklet seek path and a worklet ring rule that #1387, which now lands
first, moves to the Worker.**

- **The chain.** 06b depends on 06a. 06a now depends on #1293 (`06a…:146-147`). #1293 depends on
  #1387 and #1381 (#1293 Dependencies). So 06b lands after #1387 (06a says so itself, `:25-28`).
- **What #1387 does in `worker` mode**, which is the mode of the cross-origin-isolated first-party
  app:
  - D1: the worklet's host has no producer set.
  - D2: "The worklet's post-boot set drops the source exports".
  - D4: `miso.seek.v1` travels to the Worker.
  - D5: "The SDK's feed module is loaded into the control Worker as a module, not as a worklet
    prelude".
  - Its root ruling: seeks "belong to the control half (the Worker), never to the render worklet".
- **What 06b freezes:**
  - D1 (`06b…:44-48`): "the worklet's `receiveSessionSeek` calls `miso_engine_web_v1_session_seek`
    between blocks". That export seeks every source.
  - D2 (`:49-53`): "The engine processor records the generation of the last session seek…
    In `applySharedSeek`…". That is the worklet-prelude drain.
  - Gate 2 (`:112-114`) pins that the worklet calls the export.
  - Its Context (`:18-29`) and authorized paths (`:80-86`) describe only the routing before #1387.
  - Its own non-goal (`:93-94`, "the session seek takes the thread the source seek takes")
    contradicts D1 and D2.
- **Consequence.** If an implementer follows D1, the worklet calls a source entry point that #1387
  removed. If D2 is put in the engine processor, the drain in the Worker never sees the session
  seek's generation. It then calls the per-source export with generation `g`, gets
  `source.generation.stale`, and that ring never drains again. 06b exists to prevent exactly that
  stall (`:30-33`). Only `single` mode matches the draft.
- **Effect on the note.** The README says "No slice edits code that an earlier slice deletes"
  (`README.md:753`). That is false for 06b. This is the same class as attempt-1 M1, and the
  unrequested 06a change caused it.
- **Fix:**
  - D1: the session-seek message takes #1387's routing. In `worker` mode the Worker calls the
    export. In `single` mode the worklet's control handler calls it, outside the render-locked
    window.
  - D2: the component that calls the export records the accepted generation, and the MSB1 drain
    reads it in whichever realm #1387 D5 runs the drain.
  - Gate 2 covers both modes.
  - Add #1387 as a direct dependency, and rewrite Context and authorized paths on #1387's files.

## MINOR

- **m1. F16 narrows a root decision. Root must rule on it before slice 11 is filed. The note also
  leaves out an alternative.**
  - **The reasoning holds for the chosen layout.** With one offsets word per VCA that reaches the
    lane, a change in reach changes the word count. Decision 14 rule 1 then makes it a rebuild
    (`11…:67-72`).
  - **It contradicts a decided rule.** D15-6 (`…2026-10-05.md:145-149`, "#1247 (F9: VCA
    membership live on the C ABI)") and #1247's product outcome ("add or remove a VCA; change a
    VCA's members… with no rebuild") say membership is live. For automated lanes, slice 11 makes
    it a rebuild.
  - **Process.** The note decides this and asks root only to "record" it (`README.md:933-941`).
    Its Authority line says "nothing here reopens them" (`README.md:21`). F1's slices wait for
    root's confirmation (16a, 16b and 20 list it as a dependency), but slice 11 does not wait for
    F16.
  - **The missing alternative.** F16 rejects only a cell sized to the host's VCA cap ("3 KB per
    automated lane"). A middle layout exists and is bit-exact: one word per session VCA, in VCA-ID
    order, with `+0.0` for a VCA that does not reach the lane. Adding `+0.0` leaves every nonzero
    `f64` sum unchanged and can only turn `-0.0` into `+0.0`. `db_gain(±0) = 1` exactly, so the
    gain bits match `vca_effective_db` over the reach (`crates/session/src/vca.rs:19-34`). With
    this layout membership edits stay live, and only adding or removing a VCA rebuilds. The cost
    is `12·V_session` bytes per automated lane.
  - **Fix.** Make F16 a root decision with all three layouts and their costs. Gate slice 11 D4 on
    that decision, as F1 is gated. Correct the line at `README.md:21`.
- **m2. The list of slices that must land together is too short, and one gap can drop an
  acknowledged edit.**
  - `README.md:760-763` names four groups. It leaves out these dependencies:
    - 09a and 09b also need draft 10. Without 10, a live fader edit fights the curve, as 09b's
      Hazards say.
    - 10 needs draft 11. Without 11, an acknowledged VCA ride is not heard on automated members,
      because 10 D3 drops the `FaderDb` and 11 adds the cell write.
    - 18a and 18b, together with #1306, need draft 19. Before 19, a live effect record on an
      automated cell still reaches the lane. But amended #1306 sizes the window without that cell's
      live term, and 17b D5 and 18a D4 assume that no such record exists. So a piece can hold one
      span more than the window's capacity, and `Staged.dropped > 0`: an acknowledged edit is
      dropped.
  - Each batch is one push, so `main` is safe. But the README asks root to "file them knowingly"
    from this list.
  - **Fix.** Say that R1 and R3 cannot be split into smaller pushes, or list 09a-11 and 18a-19
    with #1306 as groups.
- **m3. New drafts 25 and 26 leave out files that their change must edit.**
  - **Draft 25** (`25…:44-45`, `:71-80`):
    - `tools/audit/src/protocol.rs:206` builds `TransportSetRequest { …, position: None }`, so D2
      breaks the audit build. The file is not an authorized path.
    - `crates/capi/src/runtime/tests.rs:2557` sets a position but is not in the inventory.
  - **Draft 26** (`26…:25-38`, `:72-82`). Each of these files is outside the authorized paths:
    - `crates/effect-contract/src/step.rs:927`, `crates/effect-compiler/tests/native_session.rs:34`,
      `crates/protocol/src/controller/tests.rs:490` and
      `crates/conformance/tests/conformance_corpus.rs:81` use the retired `Sample` rate.
    - `scripts/check-effect-runtime-policy.sh:61` pins `fn automation_segment_value(` at count 1.
      D2 deletes that function.
  - So neither slice can pass its own gates inside its own paths. The decisions themselves are
    correct: m5 below.

## NIT

- **A2 under options B, C and D, and the #1057 wording.**
  - The browser gates and A2 name command-record lowering: #1382 D2 and kinds 3, 4, 5, 16,
    `COMMAND_PAN` and `COMMAND_INPUT_FILTERS` (`README.md:326-332`; `10…:164-169`;
    `13a…:111-112`; `11…:115`; `14b…:121`).
  - The #1057 note at `45c5a1819` proposes to retire that lowering (its F4, and its #1382 row:
    "no record lowering; … by stable ID").
  - The rule survives either way, because both hosts use one shared commit. Reword the gates as
    "a browser edit through the Worker's apply" when root files them.
  - In A2, also state that under B, C or D the existing static edit keeps its meaning, and the new
    behaviour comes only through the new edit. Otherwise draft 10's gate 5 would change, and
    "changes nothing shipped" would not hold.
- **The ramp end for a zero-length ramp.** A1.4's completion sample `τ + L - 1`
  (`README.md:166-171`) is `τ - 1` when `L = 0`, which is the gate's hold after #1336. Draft 18a D2
  correctly uses `τ`, but the README does not say so.
- **Amendment wording.** `README.md:764-767` lists #1306 among specs that "land after the slice
  that needs the change". But #1306 lands before 18a, because 18a depends on it. The amendment
  comes from A7 and A8, the research that #1306 depends on, so the graph has no cycle. Only the
  sentence is wrong.
- **Stale hazards.** The hot-file hazards in `19…:140-141` and `20…:148-149` still name
  `hosts/host-web/src/lib.rs`. Neither slice now edits that file.
- **For root, not a defect of this attempt.** GitHub #1306 is CLOSED. It was closed at
  2026-10-05T13:28Z by `6b8bc7c96`, whose message "Fix #1306/#1058 …" triggered GitHub's closing
  keyword. Its spec is still open, and this plan lands it in batch R3. Reopen it.

## The attempt-1 findings, checked one by one

- **M1: resolved, except MA1.**
  - A grep of the README and all 41 drafts finds no "until #1382", no `COMMAND_REASON_*` and no
    browser admission edit. Drafts 19 and 20 no longer authorize `hosts/host-web/src/lib.rs`.
  - 09a depends on #1382 (`09a…:215-218`). So every rendering slice comes after #1382, directly or
    through 09a. Batch R1 comes after #1382.
  - **The permanent rule is correct.** A live edit of an automated cell commits its fallback value
    as `model_only` in the shared commit. Reasons:
    - Decision 14: no slot is needed for a value that is not rendered.
    - D15-17: `model_only` is one of the three paths.
    - #1315: the commanded state is the committed document, the category #1315 examined at
      `:44-47`. The reply names the path.
    - Acked-batch question: the value is committed, not dropped.
    - Why not a typed refusal: the note gives the reasons (replace, personal mix, option A), and
      they hold.
  - **Options B and C** add a field and an edit. **Option D** must be a cell word that render
    reads at events. Built that way, A10, 17b D5 and #1306's live term stay true. One wording gap
    remains (NIT 1).
  - **Dependency graph.** I parsed every draft's Dependencies section. The internal edges have no
    cycle, and no dependency is in a later batch. The README table matches the drafts; the only
    differences are transitive "brings" mentions. The only open spec that names a #1058 slice is
    #1306, as "same batch", and it does not depend on one.
- **m1: resolved.** Batch P1 contains 01, 21a, 21b, 21c, 25 and 26. It needs only #1309, and 21a
  depends on 01.
- **m2: resolved, and correct.** The route op runs before the send's compensation delay, so at
  render sample `r` it carries `timeline(r + ΣP - a(tap))`. A tap `L` ahead of the fader therefore
  changes `L` render samples earlier. After the delay, the send changes on the same timeline sample
  as the strip. Timing it at the fader's arrival would leak `L` samples. #1226 D5 still governs a
  live mute, which has no timeline sample (`README.md:495-512`, `13b…:58-81`).
- **m3: resolved** (`README.md:343-356`). The only remaining gap is NIT 1.
- **m4: resolved, apart from m2 above.**
  - 04 is split into 04a and 04b, 13b into 13b and 13c, and 21a into 21a, 21b and 21c.
  - 14a and 16a are layer building blocks that classify nothing. 14b, 16b and 20 own their
    group-cell writes, so no slice adds a rebuild rule that a later slice replaces.
  - 12 owns the jump lengths from the start. 11 adds to 10's rule.
  - 13b and 21a are still large but focused.
- **m5: resolved, and correct.**
  - 25 retires the stored transport position. 26 retires `Step`, `Linear` and `Exponential`,
    `automation_segment_value` and `AutomationRate::Sample` and its wire value 1, under the removal
    ruling.
  - No open spec uses the transport position. #1315 mentions `TRANSPORT_SET` only in its "examined"
    list and its own non-goal.
  - No open spec uses `Sample` or the moving span kinds. #1335 to #1338 touch only `Block` and
    `None`.
  - Nothing conflicts with D15-12 or D15-13 E1.
  - The inventory gaps are m3 above.
- **m6: resolved.**
  - 18a D8 reads `80 + 32·n`.
  - F12 is correct. While a lane is muted, or ramping to 0, a live fader change only remembers the
    gain, so it never cuts a mute ramp and the `-0.0` artefact goes away. During an unmute ramp it
    retargets. #1054 is open and its D10 is unchanged on `main`, so the amendment can still apply.
    Draft 08 D2 reads it.
  - The #1225 row is gone, and 13c D1 writes the cell.
- **m7: resolved in text.** [S6] has three deep pages, [S7] adds Delay-Line Interpolation, and [S8]
  uses the real section titles. I did not fetch them again; attempt 1's sub-agent verified the
  sources.
- **Nits: all resolved.**
  - Loop points are quantum-granular (`README.md:134-138`).
  - The cursor bound is `q` and is in the CPU bound.
  - 17b lists #1345.
  - The spec's attempt record names both internal reviews.
  - The three anchor ranges are fixed: `:1352-1426`, `:534-595` and `:1228-1282` are now exact.
  - F3 and F10 are as asked.

## The unrequested changes

- **06a.** Depending on #1293, following #1387's and #1381's routing, and exporting
  `session_seek_at` is correct and in scope: A1.2's anchored loop needs it in the browser. But the
  change was not carried into 06b (MA1).
- **12.** Depending on #1365 and owning the `controlSmoothing` sentence (D7) is correct and in
  scope. #1365 lands first, does not depend on #1058, and 12 only extends #1365's classifier path.

## The answers A1-A11 against the spec

- **A7** is still a formula that #1306 can evaluate at preparation: `stored(i) = |C(i)|`, and 0 for
  the EQ. It is bounded by the instance's `Block` cell count and does not read song length.
- **A8** is a clear "no".
- **A9** gives a permanent refusal. The command, the event and the queue leave the registry in
  21a-21c, with the acked-batch answer (refused at decode, and no queue remains).
- **A10** names the slices (10 removes the fader row, 20 removes the last) and the edit outcome
  (carried `rebuild`, completing `exact`; static edit `model_only`; group cell `live`).
- **A11** lists forty-one slices in twenty-six steps and seven batches. 18a and 18b render stored
  effect automation, and #1306 is in batch R3.
- OQ1 and OQ2 are self-contained: background, options with costs, and a recommendation. OQ1 does
  not decide #1057's personal-mix question (`README.md:806-808`).
- I found no new interim shortcut. The only decision made where it should have gone to root is
  F16 (m1).

## Drafts read

- **In full:** 04a, 04b, 06a, 06b, 09a, 10, 11, 12, 13a, 13b, 13c, 14a, 14b, 16a, 16b, 21a, 21b,
  21c, 25 and 26.
- **The changed parts:** 01, 02, 05, 07, 08, 09b, 15, 17b, 18a (D1-D8 and Hazards), 19, 20 and 22.

## Anchors

- I checked about 110 anchors on `6ee64f484`, most of them new in attempt 2. All are exact.
- They cover:
  - the protocol: `wire.rs`, `message_wire.rs`, `controller.rs`, `schema.rs`, `btlv.rs` and
    `protocol_corpus.rs`;
  - the effect contract and span users;
  - `crates/source` (`:51-78`, `:753-785`, `:1001-1027`, `:1154-1168`, `:1323-1365`, `:1367-1386`,
    `:1611-1736`, `:1845-1936`);
  - host-core `source.rs` and `prepare.rs`;
  - graph `lib.rs` and `runtime.rs`;
  - `IndexedRamp`, `live_route_state.rs:232-257`, the matrix stage, the fused path and `pan_matrix`;
  - the processors in builtins-compiler;
  - `live_delta.rs`, and the capi control and live tests;
  - the worklet, feed and SDK anchors of 06a and 06b;
  - `filter_control.rs`, `vca.rs`, `vca_composition.rs:81` and `plan.rs:912`;
  - the docs and rulings anchors, and #1247 `:8-10` and #1351 `:23-24`, `:48`.
- One range is short but still correct: `crates/builtins/src/lib.rs:3926-3935`. The claim is in the
  doc comment at `:3927`; the ramp check itself is at `:3938-3939`.

## Tests

No tests were added or changed. This is a docs-only research note with draft specs.

## Gates run

- **Gate 4.** `git diff --name-only 45c5a1819 50fb23b5f` lists only
  `docs/handoffs/stored-automation-1058/` (the README and drafts) and
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`.
  PASS.
- **Policy scripts.** I ran them in a `git archive` export of `50fb23b5f` at
  `/tmp/claude-1002/v1058/tree`, made into a git repository, and deleted it afterwards:
  - `bash scripts/check-workspace-policy.sh`: "workspace policy: ok", exit 0.
  - `bash scripts/check-dsp-research.sh`: "dsp research corpus: ok", exit 0.
- **Gate 1.** The A1-A11 headings and the decision-record links resolve. Each answer is a decision,
  and A2 is an owner question with options. PASS.
- **Gate 2.** PASS. A7 is a formula, A8 is "no", and A9 names 21a-21c and the permanent refusal.
- **Gate 3, as reworded.** FAIL. Draft 06b is not fileable as written (MA1), and drafts 25 and 26
  have incomplete authorized paths (m3).
- **Mechanical dependency check.** I parsed the Dependencies sections into an edge list. The graph
  has no cycle and no dependency is in a later batch.
- **Not run, by design.** No build and no benchmark; the issue is docs only.
