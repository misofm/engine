FAIL

# #1058 attempt 3: adversarial verdict (stream K, stored automation research)

Reviewed: `a1a3fe87c` on `codex/d15-stream-k` (attempt 2 was `50fb23b5f`; `c63f5f37d` between them
is the #1057 note, used only as a cross-reference). I read the files with `git show` and from a
`git archive` export at `/tmp/claude-1002/v1058/tree`. I did not touch the worktree.

Scope:
- `docs/handoffs/stored-automation-1058/README.md`;
- the 41 drafts under `proposed-specs/`;
- the #1058 spec;
- `git diff 50fb23b5f a1a3fe87c` of those paths.

Gate 3 is judged as the coordinator reworded it: each slice must have a complete, stateless,
fileable draft that lists its dependencies. F16 is judged by the coordinator's condition: it may be
a root decision if its options, costs and recommendation are complete and correct.

Attempt 3 fixes the attempt-2 major well. Draft 06b now follows #1387 in both modes, and it
records the generation and reads it in the same realm. It also fixes m2 and all five nits.

The new F16 does not meet the coordinator's condition:
- Its premise is false.
- It leaves out a bit-exact layout that keeps #1247's whole live set with no cap, at a constant
  36 bytes per lane and one `f64` add per event.
- Its recommendation narrows a live set that decision 15 decided.

That is one MAJOR. The README's mechanically checked claim, "No slice edits code that an earlier
slice or spec deletes or moves", still fails in two drafts. Draft 25 still misses a file its change
must edit. Several later slices turn earlier gates red without naming them. These are minors.

## BLOCKER

None.

## MAJOR

**MA1. F16 is incomplete. Its premise is false, and it leaves out a layout that keeps #1247's live
set whole with no cap.**

**The false premise.** `README.md:1003-1005` says: "So render needs the offsets in a cell, and every
exact layout makes some VCA edit need new memory on such a lane."
- The note's own layout 3 contradicts this. Its row at `:1019` says "Rebuild: nothing".
- The missing layout below contradicts it too.

**The missing layout ("layout 4": compose the offsets first).**
- **The rule.** Define the composition as `clamp(f64(member) + S, -144, 24) as f32`, where
  `S = Σ f64(offset)` over the reaching VCAs in ascending VCA-ID order, summed first.
- **The static path.** `vca_effective_db` uses the same rule.
- **Bit-exactness.** A per-lane precomputed `S` is then bit-exact by construction. This removes
  the only reason for one word per VCA: the claim that a precomputed sum "can round
  differently" (`09a…:84-85`, `11…:25-31`, `vca_composition.rs:81`) applies only to today's
  member-first order.
- **The cell.** One `f64` (two words) plus the ramp word, in #1312's three slots: 36 bytes per
  automated fader lane, whatever `V` is.
- **Live edits.** Each VCA edit is one live cell write: a ride, a membership change, a nested VCA,
  adding a VCA and removing one. Nothing rebuilds. No capacity is needed, compiled or configured.
- **Render cost.** Render does one `f64` add per event, not `V` (layout 2) or the reach (layouts
  1 and 3).

**What layout 4 costs.** It is a class B change to #1242's shipped order. The order is documented
at `docs/SESSION_SCHEMA_V1.md:80` ("its own value first, then the reaching VCAs' offsets"),
implemented at `crates/session/src/vca.rs:19-34` and pinned at
`crates/session/tests/vca_composition.rs:79-82`.
- **Decision 13 (a) does not fix the order.** It says only that the offset "adds to each member's
  own fader" (`docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:114-115`).
- **The vca.rs doc already reads like layout 4.** It spells the rule
  `clamp(own_db + sum(offsets), -144, 24)` (`vca.rs:5`, `:19`).
- **Which bits move.** Both orders give the same bits whenever every partial sum is exact in
  `f64`. Sums of `f32` values are exact in `f64` unless the terms differ in magnitude by more than
  about 2^21 to 2^29, depending on the term count. So only extreme values move, such as the pinned
  `1e-30` case.
- **Who rules.** It moves pinned bits, so the owner rules on it (memory
  `user-math-expertise.md`). The standing summation-order ruling covers measured speed-ups, and
  this is a liveness change.

**Layout 3's sizing is not stated correctly.**
- **C ABI.** `maximum_vcas == 0` means `maximum_tracks` (`crates/capi/src/runtime/compile.rs:517-541`;
  header `miso_engine_v1.h:191-194`; the capi test limits use 0, `ffi.rs:1338`). So
  "`12·maximum_vcas + 12`" (`README.md:1019`, `:651`) is really
  `12·maximum_tracks + 12` per automated lane on a default host, and it has no bound of its own.
- **Browser.** The host-core cap is `u64::MAX` (`hosts/host-web/src/lib.rs:6594`). The 256 bound is
  the compiled `MAXIMUM_BROWSER_VCAS` (`lib.rs:89`), enforced at boot (`:6398-6400`). Layout 3
  would make that compiled bound size per-lane render memory, and the note does not say so.
- **Draft 11 D5 is wrong for the browser.** It says (`11…:90-91`) that preparation refuses a larger
  count at `prepare.rs:1162-1170`. That is true only on the C ABI.
- **Layout 3's figure.** Its `+12` is the count word. The ramp word adds another 12 bytes, so the
  browser figure is 3,096 bytes, not 3,084.

**Why this is a MAJOR.**
- The coordinator accepted F16 as a root decision only if its options and its recommendation are
  complete and correct. They are not.
- Root is asked to rule on a set that omits the one layout that keeps D15-6's decided live set
  (#1247: "add or remove a VCA … with no rebuild", `1247…md:16-21`). That layout also costs less
  memory and less render work than the recommended one.
- The recommendation, layout 2, makes VCA add and remove a rebuild on any session with stored
  fader automation. That is a permanent narrowing of a decided item, chosen without the
  alternative in view. Under the no-shortcuts principle, the long-term design should be weighed
  before a decided outcome is cut back.
- Drafts 09a (D1, D4, gate 1) and 11 (D1, D3-D5, gate 4, and the non-goal "Any change to
  `vca_effective_db`'s order", `11…:120`) are written for layout 2. Under layout 4 they change.

**Fix.**
- Add layout 4 to F16, with its memory, its render cost, its live set and the owner ruling it
  needs (the bits it moves and the files: the schema doc, `vca.rs`, the pinned test).
- Delete or correct the "every exact layout" sentence.
- State layout 3's real sizing on both hosts, and correct draft 11 D5's browser claim.
- Derive the recommendation again. If it stays layout 2, say why layout 4 is rejected.
- Keep 09a and 11 gated on F16, and say what each changes under layout 4.

## MINOR

**m1. Attempt-2 m3 is only half resolved. Two drafts cannot pass their own gates inside their
authorized paths.**

- **Draft 25 excludes `crates/protocol/src/queue.rs`** ("`crates/protocol/**` (not `queue.rs`)",
  `25…:81`). But its product outcome and D2 remove the position from the `TRANSPORT_STATE` event,
  whose reliable payload lives in that file:
  - `ReliablePayload::TransportState { …, position: SampleTime, … }` (`queue.rs:323-335`);
  - `ReliableSlot::transport_state(…, position, …)` (`queue.rs:395-417`);
  - `controller.rs` passes `snapshot.position` into it, and `controller/tests.rs` calls it with a
    position.

  The "Every user" grep (`25…:48-53`) searched for `TRANSPORT_SET` but not for
  `TRANSPORT_STATE`, `ReliablePayload::TransportState` or `transport_state(`. Draft 21b opens
  `queue.rs` only for the `AUTOMATION_CANCELED` payload, and 21c does not touch this payload.
- **Draft 02 now adds the `automation` dependency to builtins-compiler** (`02…:140-141`) and runs
  `bash scripts/check-builtins-policy.sh` (`02…:205`). That script pins builtins-compiler's exact
  dependency list (`scripts/check-builtins-policy.sh:17-20`) and fails on the new edge. The script
  is not an authorized path. This defect is new: it comes from the 02→07 change.
- **Fix.** Add `queue.rs` (the `TransportState` payload and its constructor only) to 25's paths,
  and widen its grep. Add the builtins policy script's dependency list to 02's paths.

**m2. The claim "No slice edits code that an earlier slice or spec deletes or moves"
(`README.md:775-802`) is still false in two drafts.**

- **Draft 05 D6 conflicts with #1319 D4.** #1319 is in 05's closure: 05 → 04b → #1320 → #1319.
  - #1319 D4 already changes `audit capi`. Right after the structural transaction it calls
    `miso_engine_v1_source_seek_at(session, "fixture-source", 2, A, A)`, submits a generation-2
    quantum, and asserts `RESULT_OK`
    (`.github/ISSUE_SPECS/1319-test-held-seeks-across-swaps-and-supersession-and-add-a-seek-to-audit-capi.md:81-86`).
  - Draft 05 D6 (`05…:103-105`) adds a session seek at the same point and "submits one
    generation-2 quantum". Done literally:
    - either the session seek is stale or backpressured (the one-slot queue still holds the
      anchored seek),
    - or #1319's own seek becomes stale, and its asserted `RESULT_OK` fails.
  - 05's Context (`:51-53`) describes the audit before #1319.
- **Draft 20 D5 deletes code that draft 10 D1 keeps.**
  - 20 says "the mask line (`live_delta.rs:236`) and its rustdoc (`:181-185`) go" (`20…:84-85`).
  - Draft 10 D1 deliberately keeps that copy line (`10…:61-65`). It replaces the mask with
    `automation_row_renders` plus a new step after step 4 that returns `LiveRebuild::Automation`.
  - The masked JSON comparison runs before that step (`crates/host-core/src/live_delta.rs:228-264`, the copy at `:236`, the comparison at `:259`).
    So once the line is deleted, every automation edit returns `Structure` there. That contradicts
    20's own D5 ("An EQ automation edit is `LiveRebuild::Automation`") and its gate.
  - 20 never mentions `automation_row_renders`, which 10 D1 says "draft 20 adds the last [row]" to.
- **Fix.**
  - Rewrite 05 D6 on top of #1319 D4: for example, a session seek at generation 3 after #1319's
    anchored seek has applied, or a replacement of #1319's call that 05 states openly.
  - In 20 D5, say: the last row joins `automation_row_renders`, and either the copy line stays, or
    the per-row step folds into step 3 and keeps the `Automation` reason.
  - Then soften the README claim, or re-run its check.

**m3. Later slices turn earlier gates red without naming them.** AGENTS.md: "a change that
supersedes a test deletes it in the same PR". The authorized paths must allow that.

- **Draft 13a** unmasks row 6. Draft 10 gate 1 asserts "A mute (row 6) entry change gives `Ok` with
  no records" (`10…:151-153`). 13a does not name that case.
- **Draft 20** names only 02 gate 7, 10 gate 1 and #1335 gate 4 (`20…:94-96`). It also turns red:
  - draft 19 gate 1, "an EQ automation change stays live with no records" (`19…:147-148`);
  - #1335 gate 3, the band-gain half of
    `an_automation_on_a_prepared_effect_parameter_is_refused_before_any_ack`, which asserts
    "a block-rate target commits live: one revision, no candidate" and `pending == 0` (`main:
    crates/capi/src/runtime/live_tests.rs:2937-2948`).
- **Draft 22 D2** makes a nonzero word at the old S offset `INVALID_ARGUMENT`. That breaks #1306
  gate 2's committed test, which compiles at S = 128 and S = 4,096 (`1306…md:178-179`). 22 lists
  neither the test nor its deletion, and it limits `resource_lifecycle.rs` to "literals only"
  (`22…:82`).

## NIT

- **Table row 05** (`README.md:721`) omits #1309, which draft 05 now lists (`05…:208-209`). The
  README says each row matches its draft. All other rows match.
- **Draft 21b's authorized path** has a leftover fragment, "after #1309", on its own line
  (`21b…:95`).
- **Draft 06b D3 types.** `SessionSeekProducer.seek(frame: bigint, generation: bigint)`
  (`06b…:104`) does not match `CanonicalPcmPump.seek(frame: number)`
  (`hosts/host-web/web/stem-store/index.d.ts:206`; `pcm-pump.js:210-211` validates a number). Say
  which side converts.
- **Draft 05 D5** places its text "a paragraph after 'Starting an added stem in time'"
  (`05…:95`). #1317 D1 replaces that paragraph with one paragraph per topic.
- **Draft 07 D3** gives `value_at(t: i64) -> f32` with no receiver (`07…:78`). Draft 02 calls
  `automation::value_at` on a session entry (`02…:94`), which has no `CellProgram`. Name the
  evaluator that takes only a segment table.
- **Draft 26** leaves `RATES = {1: "sample", …}` in `scripts/check-parameter-metadata-v1.py:30`.
  This breaks nothing, but the independent metadata gate still accepts the retired rate.
- **Draft 22's Context** (`22…:26-27`) says amended #1306 D3 "keeps the density read". The
  amendment row and 21c say that S has no role and the read is gone before #1306.
- **F18** says that drafts 14a and 15 cite #1408. #1408 also changes `gain_mute_ramp_block` (its
  site 1, the fader and mute kernel). Draft 08 relies on that kernel's per-lane additions
  (`08…:30-31`) but does not cite #1408. The law stays per sample, so nothing breaks; cite it as
  14a does.
- **Memory formula.** `12·(V + 1)·F` (`README.md:656`) charges 12 bytes per lane at `V = 0`. The
  row (`:651`) and 09a say that such a lane has no cell. This over-counts, which is safe.

## The attempt-2 findings, checked one by one

- **MA1 (06b): resolved.**
  - D1 takes #1387's routing: the Worker in `worker` mode, the worklet's control handler outside
    the render-locked window in `single` mode (#1387 D2, D4; #1332 D7).
  - D2 records `lastSessionSeekGeneration` in the realm of the handler that calls the export, and
    the drain reads it in its own realm: #1387 D5's Worker drain module, or the worklet prelude.
    In each mode the handler and the drain share one thread. One host per module instance
    (#1332 D2, `LIVE_HOST`) makes a realm-local value per engine correct.
  - Gates 1 and 2 cover both modes.
  - #1387, #1332 and #1294 are dependencies, and `@internal` follows #1294 D6.
  - 06a's lifecycle refusal (#1293 D1), its `SessionState` wrapper (#1381 D2) and its
    pending-candidate gate are correct.
  - All 16 code anchors of 06b are exact.
- **m1 (F16): partly resolved.** F16 is now a root decision with three layouts and costs, the
  Authority line is corrected (`README.md:23-27`), and 09a and 11 are gated on it. The claim that
  layout 2 still rebuilds on adding or removing a VCA is correct: the cell has one word per session
  VCA, so `V` changes its size. Draft 11 D4 states the case candidly. Layout 2's `+0.0` argument is
  correct:
  - `s + (+0.0) == s` except that `-0.0` becomes `+0.0`;
  - a later `-0.0` term can only give a zero of either sign;
  - `db_gain(±0) = 1`;
  - with no reach, `v` lies in the domain, so the clamp is idle.

  But the option set is incomplete (MA1 above).
- **m2: resolved.** The four groups are 09a-09b-10-11, 13a-13c, #1306-18a-18b-19 and 21a-21c. Each
  has its reason, each sits inside one batch, and each draft states its group.
- **m3: partly resolved.** Draft 26's inventory is complete apart from the metadata NIT. Draft 25
  adds the audit tool and `tests.rs:2557`, but still misses `queue.rs` (m1 above).
- **The five nits: all resolved.**
  - The browser gates say "a browser live edit through the Worker's apply". No `kind 3`, `kind 16`
    or `COMMAND_*` gate wording remains, and `COMMAND_*` appears only in Context as today's code.
  - A2 states that the static edit keeps its meaning under every option (`README.md:361-364`).
  - A1.4 gives `τ` for `L = 0`.
  - The Acyclic wording is corrected.
  - The hot-file hazards of drafts 19 and 20 name `tests.rs`.
  - F17 asks root to reopen GitHub #1306.

## The new content

- **11 and 14b ramps.** `LiveRamps::resolve(VcaFader, …)` and `resolve(Matrix, …)` match #1394 D5-D6
  and #1247 D5. #1054 D4 gives one `LiveRampRow` per row of its D3 table, VCA offset and matrix
  included.
- **02 depends on 07.** It is in the README table and in both drafts. Both are in R1, and 07's
  package name, `automation`, follows the AGENTS.md naming rules. The new policy-pin gap is m1.
- **10 moves the `model_only_edits` cases to an EQ band-gain row, and 20 deletes them.** The two
  drafts agree. No slice between 10 and 20 unmasks the EQ: 19 D1 excludes target-preparation
  instances, and 18a gives the EQ no cell.
- **The #1306 amendment row.** It targets text that exists at `1306…md:11`, `:17-20`, `:39` and
  `:111`. After 21c, S keeps no role. Only 22's Context wording disagrees (NIT).
- **F17: correct.**
  - `gh issue view 1306` gives `CLOSED`, `closedAt 2026-10-05T13:28:43Z`.
  - The body of `6b8bc7c96` reads "Fix #1306/#1058 …".
  - The spec is still open on this branch.
- **F18: correct.**
  - The #1407 and #1408 specs exist on `5cfc1fb6d` (`codex/d15-stream-g`), and not on `a1a3fe87c`
    or `main`.
  - Both GitHub issues are OPEN.
  - #1407 D5 keeps `InputStage::apply_prepared_filter` as the entry point. Only its list of drafts
    that cite #1408 is incomplete (NIT).
- **No new placeholder and no open choice.**
  - A grep for interim, placeholder and "for now" wording finds only the README's history lines.
  - A7-A11 are still decisions.
  - The one same-batch change of a rule is stated openly and kept off `main` by a must-land-together
    group: 10 D3's empty delta for a VCA ride, which 11 turns into a cell write.
- **Owner questions.**
  - OQ1 and OQ2 are self-contained: background, options with costs, and a recommendation.
  - OQ1 says that it does not decide #1057's other personal-mix parts. The #1057 note at
    `c63f5f37d` (`:1265-1277`) defers to #1058, so the two agree.

## Dependency graph, built mechanically

I parsed the README table and the Dependencies section of every draft:
- 41 drafts and 61 internal edges;
- the graph is acyclic (a topological sort covers all 41);
- no dependency is in a later batch (P1 < R1 < R2 < R3 < R4 < P2 < Q);
- each draft's direct dependencies match its table row, apart from 05's #1309 (NIT).

A separate closure check of 02, 05, 06a, 06b, 10, 11, 14b, 20, 21a-21c, 22, 25 and 26 found no
use of #1309's moved files, of #1312's rings, or of #1382's deleted admission in an instruction.
The defects it did find are m1-m3.

## Anchors

- **HEAD code equals `6ee64f484`.** `git diff --stat 6ee64f484 a1a3fe87c -- crates hosts sdk tools scripts`
  is empty.
- **The new anchors.** All 40 anchor occurrences on lines that attempt 3 added are exact. They
  include:
  - `EFFECT_CONTRACT_V1.md:165-166`;
  - `vca.rs:96-120`, `vca_composition.rs:81`, `1247…:8-10` and `:16-21`;
  - `control.rs:1514-1543`;
  - 06b's worklet, host, feed, ring, pump and engine lines;
  - `prepare.rs:1162-1170`;
  - `lib.rs:827`, `:829`, `:835` and `:893`;
  - 25's `control_provider.rs:910`, `tests.rs:1900`, `:2403`, `:2557` and `protocol.rs:206`;
  - 26's `step.rs:927`, `native_session.rs:34`, `controller/tests.rs:490`,
    `conformance_corpus.rs:81` and `check-effect-runtime-policy.sh:61`.
- **My own reading.** I read about 30 of them myself: the 16 in 06b, the F16 anchors,
  `decision-15 :26-28`, `#1226 D3` and `#1309 D8`.
- **Claims, not anchors.** Two claims are wrong: draft 11 D5's browser claim (MA1), and 22's
  Context wording (NIT).

## Tests

No tests were added or changed. This is a docs-only research note with draft specs.

## Gates run

- **Gate 4.** `git diff --name-only c63f5f37d a1a3fe87c` lists only
  `docs/handoffs/stored-automation-1058/` (the README and 26 drafts) and
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`.
  PASS.
- **Policy scripts.** I ran them in a `git archive` export of `a1a3fe87c`, made into a git
  repository at `/tmp/claude-1002/v1058/tree`:
  - `bash scripts/check-workspace-policy.sh`: "workspace policy: ok", exit 0.
  - `bash scripts/check-dsp-research.sh`: "dsp research corpus: ok", exit 0.
- **Gate 1.** The A1-A11 headings and the decision-record links exist. A1.5's VCA layout goes to
  root through F16, which the coordinator accepts if F16 is complete. It is not (MA1).
- **Gate 2.** PASS. A7 is a formula, A8 is "no", and A9 is a permanent refusal that 21a-21c retire.
- **Gate 3, as reworded.** FAIL:
  - F16 gates 09a and 11 on an incomplete decision (MA1);
  - drafts 02 and 25 cannot pass their gates inside their paths (m1);
  - 05 D6 and 20 D5 conflict with their closure (m2).
- **Not run, by design.** No build and no benchmark; the issue is docs only.
