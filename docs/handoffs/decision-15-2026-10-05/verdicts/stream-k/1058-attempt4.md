PASS

# #1058 attempt 4: adversarial verdict (stream K, stored automation research)

Reviewed: `d79bc9cb3` on `codex/d15-stream-k` (parent `423b9d4a1`, the #1057 fold; attempt 3 was
`a1a3fe87c`). I read the files with `git show` and from a `git archive` export of `d79bc9cb3` at
`/tmp/claude-1002/v1058/tree`, which I deleted after the review. I did not touch the worktree. I
ran no build.

Scope:
- `git diff a1a3fe87c d79bc9cb3` of `docs/handoffs/stored-automation-1058/` and the #1058 spec;
- the full README (1,297 lines) and all 41 drafts;
- the specs the changed drafts name (#1242 text in code, #1247, #1306, #1309, #1317, #1319, #1323,
  #1335, #1382, #1387; #1408 is not on this branch, F18).

Two root decisions bind this review:
- F16 is layout 4 by root's ruling. I judge only whether the note and drafts record and implement
  it correctly.
- Gate 3 is "each slice has a complete, stateless, fileable draft spec that lists its
  dependencies".

Attempt 4 resolves the attempt-3 major and all three minors and nine nits. F16 records root's
ruling correctly and the same way in every place. The bit-move bound is correct. Drafts 09a and 11
implement layout 4 exactly, on the static path and the render path. 09a rewrites the pinned test in
the same slice. The dependency graph is acyclic, and it matches the table.

I found no BLOCKER and no MAJOR. Four MINOR findings remain:
- three are new text from the m1 fix;
- one is a stale-oracle gap that the order change leaves.

Root can fold each of them when it files the drafts.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. The six relocated realtime gates say that they run a commit that host-core does not contain.**
- **Where.** `11…:138-143`, `13a…:117-121`, `13b…:144-148`, `14b…:134-138`, `15…:144-148` and
  `16b…:145-149`. Each says: "It drives the script through host-core's shared commit and render
  session, the code the browser Worker and the C ABI both run."
- **Where the commit is.** The shared commit is `crates/control-plane`'s live commit
  (`control_plane::SessionState`; #1309 D1-D9; #1382 lines 43-47, 79, 90 and 129).
  `crates/control-plane` depends on host-core.
- **Why a host-core test cannot run it.** A host-core integration test can drive the commit only
  through a new `host-core` → `control-plane` dev-dependency, which is a dev cycle. No draft
  authorizes `crates/host-core/Cargo.toml` for it.
- **The scripts need the commit.** Gate 1 of draft 11 moves a VCA by a transaction. Draft 13a's
  terms write comes from the commit. Draft 11's own offsets-cell write is in
  `crates/control-plane/src/` (`11…:94`).
- **What an implementer sees.** Either an edit outside the paths, or a test that writes the cells
  itself. That test checks the render-thread claim (allocations == 0), but it does not run "the code
  both hosts run".
- **Fix.** Choose one:
  - put the binaries in `crates/capi/tests/`, where capi already has the dev-dependency
    (`crates/capi/Cargo.toml:26`) and drives the real commit;
  - put them in `crates/control-plane/tests/`, and authorize that manifest;
  - keep host-core, and reword each gate to "drives host-core's preparation and render session, and
    writes the cells that the commit writes".
- **Why it is not a MAJOR.** The measured property stays testable inside the authorized paths.
  This is the same class as attempt-3 m1.

**m2. Three member-first oracles outlive draft 09a, so the README overstates its claim.**
- **The claim.** `README.md:1061-1063` says that 09a rewrites the pinned test, "so no test pins the
  old order after it lands".
- **The three oracles.** Each states and computes the member-first order that 09a D6 retires:
  - `crates/host-core/tests/vca.rs:268-279`, `reference_db`. Its doc says "Decision 13 (a) as the
    test states it: `own + offsets` in `f64`, the offsets in ascending VCA ID order";
  - `hosts/host-web/src/tests.rs:13360-13371`;
  - `hosts/host-web/src/tests.rs:13697-13714`, `vca_reference_effective`.
- **They stay green.** They compare rendered PCM. The new order moves a dB value only in degenerate
  cases. For example, `in_domain` (`crates/dsp-reference/src/randomized.rs:292-310`) draws the
  smallest subnormal and the ±12 edges, which give `tiny` against `0.0`, and `db_gain` of both is
  exactly 1.
- **Why they still matter.** Each oracle then documents a superseded rule.
  - 09a does not authorize `crates/host-core/tests/vca.rs`.
  - 09a authorizes `hosts/host-web/src/tests.rs` only for "one test" (`09a…:150-158`).
- **Fix.** Add the three references to 09a's paths and to Deliverable 3, with the order rewritten
  in the same slice. Then soften the README sentence, or make it true.

**m3. The `memset_pattern16` risk is real, but no draft names its rule. The trap-owner risk does
not apply.**
- **The trap owners do not apply.** `scripts/check-web-audioworklet-callgraph.py` follows only
  direct `call N` edges (`CALL`, `:96`). Render reaches every new piece of code through
  `call_indirect`:
  - the plan executor (`crates/engine/src/realtime/plan.rs:550`,
    `Option<Box<dyn PreparedPlanExecutor>>`);
  - every bank stage (`crates/rack/src/lib.rs:1470`, `Box<dyn BankStage>`);
  - every effect (`crates/graph/src/lib.rs:884`, `Box<dyn PreparedNativeEffect>`).

  No draft adds Rust on the direct path from `miso_engine_web_v1_render`. 06a's export is a
  control export with its own list entry. So no slice can add a trap owner to the closure that the
  gate walks. Draft 20's hazard (`20…:170-172`) is conservative and does no harm.
- **The memset risk is real.**
  - The required `cross-target` job (`.github/workflows/qualification.yml:889-915`) counts
    `bl _memset_pattern16` in each product crate's whole iOS release assembly, not in a closure.
  - It fails on a rise above a ceiling, and on any call in a crate that has no row
    (`scripts/lib/aarch64-known-defects.py:136-152`).
  - These crates have no row: `automation`, `builtins-compiler`, `rack`, `source` and
    `effect-compiler` today, and `control-plane` once #1309 adds it.
  - A new call site of an inlined four-lane kernel, for example a split piece path beside the fused
    path (`09b…:54-55`), can add calls.
- **Why it is not unpassable.** The fix is in code, inside each slice's own crates. Raising a
  ceiling would add a libc call in render, so no draft must authorize it.
- **The gap.**
  - These drafts change product-crate code but do not run `bash scripts/check-cross-targets.sh`:
    08, 09a, 09b, 10, 11, 12, 17b, 18a, 18b, 19 and 20.
  - No draft in batches R3 and R4 runs it at all, so the first signal is that batch's CI push.
- **Fix.** Add the script to those gate lists, at least 17b, 18b and 20. State the rule as draft 20
  states it for traps: "add no `memset_pattern16` call; fix it in code, never by a ceiling".

**m4. Draft 24a's unsafe-owner widening is necessary and complete, but too broad. Its gates also
leave out the scripts it edits.**
- **Necessary.**
  - The row times the C ABI entry points, which are `unsafe extern "C"`
    (`crates/capi/src/ffi.rs:311`, `:426`, `:807`).
  - The workspace denies `unsafe_code` (`Cargo.toml:90`).
  - Both owner scans see the file: `scripts/check-bench-policy.sh:219-233` and
    `scripts/check-realtime-policy.sh:29`.
- **Complete.** The authorized edits cover both lists, both self-tests
  (`scripts/test-bench-policy.sh:499` and the realtime fixtures) and
  `docs/REALTIME_DEPENDENCY_POLICY.md:45`.
- **Too broad.**
  - It exempts all 3,067 lines of `tools/bench/src/console.rs`, which hold every console row,
    from both scans.
  - `check-bench-policy.sh:211` says that a new owner file "is a new unsafe ownership boundary and
    needs a decision".
  - The audit keeps its FFI in a dedicated `tools/audit/src/capi.rs`.
  - A dedicated module that holds only the C ABI calls (for example
    `tools/bench/src/console_capi.rs`) is the narrower boundary.
- **The gates.** 24a's commands (`24a…:172-180`) run none of the six policy scripts whose pins it
  edits:
  - `check-bench-policy.sh` and `test-bench-policy.sh`;
  - `check-realtime-policy.sh` and `test-realtime-policy.sh`;
  - `check-conformance-boundaries.sh` and `test-conformance-boundaries.sh`.

## NIT

- **Draft 09a gate 1** (`09a…:194-196`, test value `:234-235`) asks for offsets "so that the two
  orders … give different `f64` sums". That does not make the render bits differ.
  - Example: `-7.25` with `[16, 2^-49, -16]` gives two different `f64` sums and the same `f32` dB.
  - So the gate does not show that render and the static path use one order.
  - Ask for different gain bits, or leave the order to gate 6, as `11…:170` does.
- **Draft 11 gate 2** (`11…:126-128`, test value `:163-164`) uses two nested VCAs "whose two
  summation orders differ".
  - With two offsets, `S = o_v + o_w` is commutative, so no nesting order can change it.
  - Use three VCAs, or drop the claim.
- **Draft 05 D6** (`05…:109-116`) should state why `declare_discontinuity` publishes nothing on the
  audit session:
  - no floor is raised, and the source-read offset is 0 (#1323 D2);
  - so #1319 D4's liveness witness `replacements == 1` (`tools/audit/src/capi.rs:521-524`) stays.

  This is true for the audit's `UpsertTrack` transaction, but the draft does not say it.
- **Stale function name.** `README.md:474` (A5) and `09a…:104` (D2) still name `vca_effective_db` as
  render's conversion. Under F16, render calls `vca_compose_db`; `09b…:57` was updated.
- **F16 Authority** (`README.md:1048-1052`) cites a private memory file. No repo reader can open
  it. At filing, cite root's decision-15 row.

## The attempt-3 findings, checked one by one

**MA1 (F16): resolved, by the root ruling.**
- **The false premise is gone.** Layout 4 is the decision (`README.md:1036-1047`).
- **The rule.** The static path and the cell use one rule: `clamp(f64(member) + S, -144, 24)`. `S`
  is the `f64` sum of the reaching offsets in ascending VCA-ID order, from the first offset. An
  empty reach returns the member unchanged.
- **Memory and edits.** The cell is 36 bytes per automated lane at any VCA count. Every VCA edit is
  a live write.
- **Authority.** The authority, and the measured-speed-up caveat for owner review, are recorded
  (`:1048-1052`).
- **The amendment.** The amendment is slice 09a's (`docs/SESSION_SCHEMA_V1.md:80`,
  `vca_composition.rs:79-82`; `09a…:119-133`, gate 6 `:211-216`).
- **Layouts 1-3** are rejected alternatives. Layout 3's figures are right:
  - `12·V_cap + 24`;
  - `3,096` bytes in the browser;
  - `maximum_vcas == 0` means `maximum_tracks` (`compile.rs:517-541`, `miso_engine_v1.h:191-194`,
    `ffi.rs:1338`);
  - the browser bound is `MAXIMUM_BROWSER_VCAS` (`lib.rs:89`, `:6398-6400`), and its host-core cap
    is `u64::MAX` (`:6594`).
- **Consistent everywhere.** The README Authority, A1.5, the Costs row and formula (`36·F`), F16,
  the table rows of 09a and 11 (no "F16 decided" dependency), the #1058 decision record (A4) and
  the attempt record all say the same. No pre-ruling wording remains outside the history.
- **The bit-move bound is correct.**
  - Each term is an `f32` in `[-144, 24]` (`validate.rs:584`), so every partial sum is a multiple of
    `Q = ulp_f32(m_min)`, and its size is at most `144k`.
  - If `m_min ≥ k·2^-20`, then `2^53·Q > 2^9·k > 144k`. So both orders are exact in `f64`, give
    one value and give the same bits.
  - Sign of zero: the result is `-0` only when every term is `-0`, in either order.
  - The stated threshold is safe by a factor of 2 (`k·2^-21` is enough).
  - The note states the bound as a sufficient condition, which is correct.
- **The crafted cases are right.**
  - `(24, [-24, 1e-30])` gives `0.0` (today `1e-30`).
  - `(1e-30, [24, -24])` gives `1e-30` (today `0.0`).
  - `(0, [24, -24, 1e-30])` gives `1e-30` in both orders. It pins the offsets' given order, as
    labelled.
  - `:82` stays `0.0`.
- **Drafts 09a and 11 implement layout 4 exactly.**
  - `vca_offsets_sum` gives `None` for no offset, else the `f64` sum from the first offset.
  - `vca_compose_db` adds the member, clamps and rounds once.
  - `vca_effective_db` returns the member bit for bit for no offset, else it calls
    `vca_compose_db`.
  - The cell seed is `S`, or `+0.0` for an empty reach. The `+0.0` argument holds: the curve lies in
    the domain (draft 02), and `db_gain(±0) = 1`.
  - Render computes `checked_fader_gain(vca_compose_db(v, S))`.
  - In 11 D4, the classifier computes `S` with the same function. A change of the `S` bits gives one
    slot write, with `LiveRamps::resolve(VcaFader, …)`. Each of these is live: a ride, a membership
    change, a nested change, adding a VCA and removing one.
  - In 11 D3, render compares the composed gain, so a `±0` change of `S` gives no jump.
  - Every static caller reaches the one function: host-web's live path uses
    `host_core::LiveVcaState` → `vca_effective_db` (`hosts/host-web/src/lib.rs:926`), so the
    browser's live path takes the new order too.

**m1: resolved, with new text that makes MINORs m1 and m4.**
- Draft 25 authorizes the `TransportState` payload and constructor in `queue.rs`.
- Its widened grep finds exactly its file list. I re-ran it on the export.
- Its anchors `queue.rs:323-335` and `:395-417`, and `controller.rs:3040`, `:2688` and `:3398`, are
  exact. So are `controller/tests.rs:3206` and `:3416`.
- Draft 02 authorizes `expected_compiler` (`check-builtins-policy.sh:17`), which
  `gate_toml_dependencies` sorts (`scripts/lib/gate.sh:162`).

I sampled the policy authorizations of 12 drafts against the scripts:

| Draft | What I checked |
|---|---|
| 02 | the builtins pin |
| 04a | the exhaustive `diagnostic` match (`host-core/src/source.rs:78-104`) and `SourceSeekError` without `non_exhaustive` (`source/src/lib.rs:381-393`); `source_diagnostics.rs:27` and `:176-198` |
| 04b | host-web has no `graph` dependency (`:25-33`); host-core's `test-support` (`:16`) |
| 05 | the frozen symbols (`check-capi-abi.sh:192-208`); capi's `graph` dev-dependency (`:25-27`) |
| 07 | `g6_full_corpus_ftz.rs:188-192`; `is_width_dependent` (`wasm-gate-corpus:527-529`) |
| 13b | the graph pin (`check-graph-policy.sh:19-22`) and its fixture (`test-graph-policy.sh:7-13`); builtins' reverse-dependency ban (`:21`) |
| 16a and 16b | `FILTER_DESIGN_CALLS` is `#[cfg(test)]` (`builtins/src/lib.rs:723-724`, `:829`); builtins-compiler's `test-support` enables `builtins/test-support` |
| 18a | the effect-compiler pin (`check-effect-runtime-policy.sh:12`) |
| 20 | `maximum_targets()` (`prepared_target.rs:67`), the private `MAXIMUM_TARGETS` (`control.rs:24`) and the rack pin (`check-rack-policy.sh:23`) |
| 21a and 21c | capi tests `live_tests.rs:1379-1407` and `tests.rs:1915`, `:2370` |
| 24a | m4 |
| 26 | `RATES` (`check-parameter-metadata-v1.py:30`, `:477`) and its self-test (`test-web-audioworklet.sh:205`) |

- The host-core `bench-support` dev-dependency (`Cargo.toml:37`) and the hosts ban
  (`check-bench-policy.sh:257-280`) are exact.
- Each authorization matches its script. Every gate can pass inside its paths, apart from m1's
  wording, and m3, whose fix is in code.

**m2: resolved.**
- **05 D6** builds on #1319 D4, which I re-read (`1319…md:81-86`). #1319 seeks at call 1
  (`CALLS_BEFORE_TRANSACTION = 1`, `tools/audit/src/capi.rs:55`) and anchors at `4·Q`, so call 4
  applies it. 05 then:
  - declares a discontinuity;
  - seeks the session at generation 3 to `B = 8·Q`, which is strictly above every consumer's
    generation;
  - submits one generation-3 quantum.

  So no slot holds a pending seek. See the NIT above for the witness.
- **20 D5** keeps the copy line (`live_delta.rs:236`) as 10 D1 does, and adds the EQ rows to
  `automation_row_renders`. 10 D1's step still returns `Automation`.
- **The README claim** now states what each attempt checked, and what it did not check
  (`README.md:777-822`).

**m3: resolved.**
- 13a rewrites draft 10 gate 1's mute case inside its paths.
- 20 rewrites these cases inside its paths:
  - 02 gate 7;
  - 10 gate 1;
  - both halves of #1335 gate 4;
  - 19 gate 1's EQ case;
  - the band-gain half of #1335 gate 3.
- 22 D5a deletes #1306 gate 2's test (`1306…md:178-179`), and `resource_lifecycle.rs` is authorized
  for that.
- A grep of the drafts for gates that pin a masked row as "no records" finds no other case.

**The nine nits: all resolved.**
- Row 05 lists #1309 and #1319.
- 21b's stray fragment is gone.
- 06b takes a `number` frame and refuses a frame above `MAX_SAFE_INTEGER`. The pump is
  `seek(frame: number)` (`index.d.ts:206`), validates at `pcm-pump.js:210-211`, and keeps a `bigint`
  counter (`:212`).
- 05 D5 places its paragraph in #1317 D1's sequence, which has a `seek_at` paragraph
  (`1317…md:57-60`).
- 07 D3's `value_at(segments, t)` matches 02's call.
- 26 removes the `RATES` entry and runs the self-test.
- 22's Context matches the #1306 rows.
- 08 cites #1408, and F18 says so.
- The memory formula is `36·F`.

## Dependency graph, built mechanically

I parsed the Dependencies section of every draft, the first sentence of each bullet without
parenthetical asides, and compared it with the README table:
- 41 drafts and 61 internal edges;
- each draft's direct draft and spec dependencies equal its table row. The one exception is #1315
  for 21a, which the row gives as the ordering "no later than #1315's C ABI half";
- a topological sort covers all 41, so the graph is acyclic;
- no dependency is in a later batch (P1 < R1 < R2 < R3 < R4 < P2 < Q).

Two edges that a naive parse finds (21a→21c, 21c→21a) come from explanatory text, not from
dependencies.

The closure checks for the drafts that this attempt changed:
- **25 against 21b and 21c.** They touch different `queue.rs` payloads.
- **21a and 21c against #1309 D8.** The capi tests stay in capi.
- **04a's diagnostic arm against #1316, #1318 and #1320.** They add arms, and 04a adds one more.
- **The edges of 13b, 17b and 18a.** Graph's `automation` edge is 13b's (R2). 17b in R3 relies on
  it.
- **05 against #1319 and #1323.**
- **09a and 11 against #1247, #1312 and #1382.**

None of these edits code that an earlier-landing slice or spec deletes.

## The new risks that the implementer flagged

- **New trap owners in the worklet call graph.** These do not make any gate unpassable. Render
  reaches every new piece of code through `call_indirect`, which the gate does not follow (m3).
- **New iOS `memset_pattern16` call sites.** A real, required gate judges them, but each slice can
  fix them in its own crates. A ceiling raise must not be authorized. This is MINOR m3: a missing
  gate line and rule, not a missing authorization.
- **Draft 24a's unsafe-owner widening.** It is necessary and correct for both scans, but wider than
  it needs to be (MINOR m4).

## Regressions, placeholders, anchors

- **No placeholders.** `git diff --stat 6ee64f484 d79bc9cb3 -- crates hosts sdk tools scripts` is
  empty. A grep of the added lines for interim, placeholder, "for now", TODO and open-choice wording
  finds none. A7-A11 are still decisions. The conditional authorizations, such as "only if draft 05
  has not landed" and "only if D7 crosses the ceiling", are bounded and closed.
- **OQ1 and OQ2** are unchanged and self-contained.
  - OQ1 says that it does not decide #1057's other parts (`README.md:897-899`).
  - The #1057 note on this branch (`1057-design-note.md:1339-1351`) says that its part 3 "follows the
    owner's answer to #1058 question 2", and gives a "Recommendation (not a decision)".
  - So the personal-mix question stays undecided.
- **Anchors.** I read about 60 anchors myself, most of them new in this attempt. All are exact on
  `d79bc9cb3`, whose code equals `6ee64f484`'s. They include:
  - the F16 set (`vca.rs:5`, `:19-34`, `:96-120`; `vca_composition.rs:79-82`; `validate.rs:584`;
    `SESSION_SCHEMA_V1.md:80`; the ruling `:114-115` and `:129`; `1247…md:8-10`, `:16-21`;
    `compile.rs:517-541`; header `:191-194`; `ffi.rs:1338`; `prepare.rs:1162-1170`; host-web
    `:89`, `:6594`, `:6398-6400`);
  - draft 25's six;
  - the anchors of 04a, 05, 06b, 07, 13b, 16a, 18a, 20, 21a, 21c, 22, 24a and 26 that are listed
    above.

  One small point: `SESSION_SCHEMA_V1.md:80`'s sentence runs on to `:81`. 09a's "`:80` only" means
  that sentence.

## Tests

No tests were added or changed. This is a docs-only research note with draft specs.

## Gates run

- **Gate 4.** `git diff --name-only 423b9d4a1 d79bc9cb3` lists only
  `docs/handoffs/stored-automation-1058/` (the README and 28 drafts) and
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`.
  PASS.
- **No code moved.** `git diff --stat 6ee64f484 d79bc9cb3 -- crates hosts sdk tools scripts` is
  empty.
- **Policy scripts.** I ran them in a `git archive` export of `d79bc9cb3`, made into a git
  repository:
  - `bash scripts/check-workspace-policy.sh`: "workspace policy: ok", exit 0;
  - `bash scripts/check-dsp-research.sh`: "dsp research corpus: ok", exit 0.
- **Gates 1 and 2.** PASS. The A1-A11 headings and the decision-record links exist. A7 is a
  formula, A8 is "no", and A9 is a permanent refusal that 21a-21c retire. F16 is recorded as root's
  decision.
- **Gate 3, as reworded.** PASS. Each of the 41 slices has a stateless, fileable draft with its
  dependencies. The MINORs are filing-time edits.
- **Mechanical checks.** The dependency graph script, the grep of draft 25's users and the other
  greps cited above.
- **Not run, by design.** No build, no benchmark, no wasm or iOS assembly. The trap-owner and
  memset judgements above come from reading the gate scripts and the dispatch types, not from an
  artifact.
