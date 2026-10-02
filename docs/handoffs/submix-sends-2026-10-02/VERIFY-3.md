# VERIFY-3: third adversarial review (diff-focused) of the submix, send and VCA plan (revision 2)

Role: Sol adversarial review, third verifier. I wrote none of the plan. I reviewed `REVISION-2.md`,
`DESIGN.md` (revision 2) and every body in `issues/` (00-28, BM1-BM3, V0-V5) against `main` at
`fe8ac679` (confirmed with `git log -1`), `AGENTS.md` and the owner preference files. Nothing in the
repository or on GitHub was modified.

## Verdict: PASS-WITH-FIXES

- **VERIFY-2 findings.** Both BLOCKERs are resolved. Of the 19 MAJORs, 18 are resolved in full. M9
  (gate commands) is resolved for every item VERIFY-2 listed, but its defect class recurs in new
  places (section 1).
- **REVISION-2's own calls.** Its rejections and refinements are sound (section 1.2). The design
  needs no new round.
- **New defects.** Renumbering and splitting introduced, or left, **one new BLOCKER and
  eleven new MAJORs** (section 3). Every one has a mechanical spec-level fix that changes no
  architecture.
  - The BLOCKER is the N2 class again, on the SDK side: after the K2 push, every SDK
    `observationMap()`/`readObservations()` call throws for a session whose bus effect declares a
    tap.
  - **The BLOCKER and MAJOR fixes are conditions of the PASS.** Apply them while landing the specs;
    no further design or verify round is needed.
  - MINORs (section 4) may be applied at landing or left to the implementer.
- **Structure.** The dependency DAG is acyclic. Every dependency line names an exact published
  title. Every DESIGN 9.1 title equals its file's H1. K0, K1, K3 and C1 leave `main` coherent; K2
  does not until B1 is fixed, and K1/K3 leave stale `AGENTS.md` qualifiers until N-D is fixed.

### How the evidence was gathered

- **Own reading** of all 38 bodies and of DESIGN sections 1-2 and 5-10.
- **Direct checks on `main`** of about 110 anchors, starting with the ones REVISION-2 introduced or
  moved:
  - the attach functions and their callers, host-web filing and kind-4 composition;
  - `reduce_plane`, `bank_gather_source`, `NodeKind::Route` and its arm, `PlanningMetadata`,
    `plain_route_gains`, and `route_fold`'s master conditions;
  - the 17 `PreparedRoute` literals, the 38 `GraphBuiltinsCompileRequest` literals, the
    `HostPrepareCaps`, `CompileLimits` and `HostSessionShape` literals, and `limits_are_valid`;
  - `WebMeterHeader`, the session-map field list, the export lists, the reason and kind vocabularies
    and their mutations;
  - the phase-profile constants, every BM1/BM2 runner pin, the slice 27 capi anchors, and the V1/V2
    anchors.
- **A scripted cross-reference check**:
  - every DESIGN 9.1 title equals its file's H1;
  - every italic cross-reference resolves to an H1, or to #1053's exact GitHub title (checked with
    `gh issue view 1053`);
  - every body's Dependencies section matches DESIGN 9.1/9.4.
- **Four read-only sub-verifiers** (Opus 5.5 xhigh):
  - anchors in 02-08 (69 checked), 09-17 (about 70) and 18-26 (about 70);
  - every gate command in every body and in DESIGN 7: 75 distinct command forms, about 500 uses. The
    Python gates and `ci-path-router.py` were run directly.
  - I re-checked on `main` every finding of theirs that this document relies on.

---

## 1. VERIFY-2 findings

| ID | Status | Evidence |
|---|---|---|
| N1 `follows_mute` into the output | **RESOLVED** | Slice 20 D1/D5/gate 6; DESIGN P11, 5.6. The code is right: `DiagnosticCode::InvalidEnum` is `schema.invalid_enum` (`diagnostic.rs:37`, `:90`), and `validate.rs:646-652` is the cited context precedent. Slices 26-28 carry "output routes never follow"; BM1 writes `false` on output routes. |
| N2 K1 breaks live-controlled boots | **RESOLVED** | Slice 03 D6, gates 5-6; slice 05 gate 7; slice 10 D5/gate 1. Both attach functions walk every entry (`effect-compiler/src/prepare.rs:1457-1641`, `prepared.session` is a `CompiledSession`), host-core calls them at `prepare.rs:887-911`, and host-web refuses a miss at `lib.rs:5921-5927`/`:5990-5995`. **But the same class recurs at K2 in the SDK (new BLOCKER B1).** |
| M1 `route_coefficients` placement | **RESOLVED** | Slice 18 D2: `graph::gated_route_coefficients` plus `graph_compiler::route_coefficients`; `check-graph-policy.sh` is untouched. No `graph::route_coefficients` survives (grep). |
| M2 prepared carrier | **RESOLVED** | `PreparedRoute.gate: RouteGate`; exactly 17 literals (`git grep 'PreparedRoute {'` gives 18 lines with the definition); canonical `route-mute` and `route-follow-zeroed` rows are written only when set. |
| M3 live route vs fold | **RESOLVED** | Slice 22 D6 and gate 7. `route_fold`'s master conditions (`runtime.rs:6351-6420`) confirm the hazard. |
| M4 inline kind-4 composition | **RESOLVED** | Slice 14 public `effective_mute`; slice 16 D3 and gate 3. The inline copy is `lib.rs:4372`. |
| M5 unsorted strip list | **RESOLVED** | Slice 10 D2 `strip_index`; the three searches are `lib.rs:5078`, `:5924`, `:5993`. |
| M6 K2 gate order | **RESOLVED** | Tables move to slice 10; the bus gain-reduction and bus-master gates sit in 16; slice 11 gate 3 is the acceptance/echo/refusal form. |
| M7 ID staging | **RESOLVED** | Slices 13 D2 and 25 D2; `HostSessionShape {` is built only at `shape.rs:50`. |
| M8 authorized-path gaps | **RESOLVED** for every listed item | Each listed path is in its new slice. The class recurs: N-G (06 → host-web tests), N-J (13 → `live-controls-evals.mjs`), N-F (18 → `enginectl-cli.mjs`). |
| M9 gate commands | **PARTLY RESOLVED** | Every listed item is fixed (argument forms, `qualification.yml:710`, foreign widths, `engineCanonical()`, `insert(..)`, `.mute` refusal path, browser gates on 26, the determinism artifact). The class recurs in new MAJORs: brace-expanded policy lines (N-A), a vacuous `--kernel-min` (N-B), the writer-parity eval in `writer-evals.mjs` again (N-F), `headless-path-evals.mjs` (N-I), and a browser-only projection gated in a headless eval (N-J). |
| M10 slice 01 path rule | **RESOLVED** | `collection_path()` at the four `"$.tracks"` literals (`compile.rs:249`, `:287`, `:293`, `:294`, verified); sidechain path `:377`; `EffectPreparedEntry.strip_path`, whose only literal is `prepare.rs:596`; the five builders `:4777`-`:4850`. |
| M11 #1053 coordination | **RESOLVED** | Slice 00 annotation (submix fields, follow sources, VCA, both renames); 02 D8, 10 D6, 14 D4, 20 D6. |
| M12 VCA prepared follow-mute | **RESOLVED** (drafted) | V2 D1/D2 and gate 3. |
| M13 harness hides defects | **RESOLVED** | DESIGN 7 "Signals in live-route gates"; slices 16, 22-26. |
| M14 rename churn | **RESOLVED** | P17. No slice renames a wire or SDK spelling (grep). Keeping host-core's `master_track` is fine. |
| M15 slice size | **RESOLVED**, one outlier | The splits were made. Slice 18 is still about a day of work (MINOR 4.1). |
| M16 slice 24 unmeasurable | **RESOLVED** | BM3 uses `graph::test_only_phase_profile`: `ROUTE` = 3, `OUTPUT` = 4, `IDENTITY_COPY` = 5, `OTHER_OP` = 7 (`graph/src/lib.rs`), rack `FOLD` (`rack/src/lib.rs`), harness `gain_pan_profile.rs:212`, and `test-support` on console-workload's dev dependency. No decision is filed. |
| M17 benchmark wiring | **RESOLVED** (not merged) | BM1 has the short-run record test (`console.rs:2841`), the `console_model()` arm and the named fact sources. BM2 reshapes the browser section (`:1573` case, `prepare` `:84`, pairwise checks). All BM1/BM2 runner pins verified. |
| M18 C ABI ramps | **RESOLVED** | Slice 27 D2 (ruled ramp for every record; per-change smoothing only where the wire carries it), D6 (A1.2), gate 1 at four rates. |
| M19 VCA bodies | **RESOLVED** | V1 paths (`support.mjs:96`, `tests/abi_layout.rs:166`, `round_trip.rs:293`, `:699`). V2 carries M12; V3 runs the documented argument forms. V1 repeats N-F's `writer-evals` placement. |

### 1.2 REVISION-2's rejections and changes

1. **BM1/BM2 not merged: agree.**
   - Each owns a disjoint section of `test-console-benchmark.sh` (`:1-1423` and `:1424-1590`).
   - BM2 depends on BM1's `sends_console_fixture`.
   - `main` is coherent after BM1 alone: the web section and `web_documents_valid` do not change
     until BM2.
   - VERIFY-2's reason for merging (a shared file, and BM2 turning a required script red) is fully
     answered.
2. **Extra splits** (09→09/10, 25→27/28, V3→V3/V4): **agree.** Each seam leaves an independently
   gated outcome. The new 09/10 boundary is clean: 09 is caps and the C ABI only.
3. **MINOR 24 rejected: correct.** `ffi.rs:1155-1161` builds an `EngineConfig`, and `:1353` mutates
   that `EngineConfig`'s `reserved[2]`. The four `CompileLimits` literals are exactly
   `resource_lifecycle.rs:160`, `runtime/tests.rs:17`, `ffi.rs:1163` and `tools/audit/src/capi.rs:238`.
4. **N1 refusal code `schema.invalid_enum`: agree.**
   - The session has no boolean-domain code; the 19 codes in `diagnostic.rs` were read.
   - `numeric.out_of_schema_range` is numeric, and `schema.wrong_type` would misreport a well-typed
     value.
   - A closed value illegal in context is exactly the `validate.rs:646-652` use.
   - No temporary code is allocated.
5. **N2 interim in effect-compiler, removed in slice 10: agree.**
   - Both attach functions walk `prepared.entries` themselves, so a host-core filter would have to
     change them anyway.
   - Placing the rule there covers every caller, and the removal touches two functions.
   - The partial-control bank precedent exists.
   - The one gap is the SDK, which the interim never covered because no bus binding exists before K2
     (B1).
6. **Keeping `master_track`: agree.** It is internal, consistent with P17, and leaves eight test files
   alone.

---

## 2. Structure, DAG and batch boundaries

- **Titles.** All 38 H1s equal DESIGN 9.1-9.3, and every italic reference resolves. The only line
  breaks inside a title are in slice 00's blockquote, where `> ` continuations render correctly.
- **DAG**, extracted from every Dependencies section:
  - 00→01→…→08, then 09←08, 10←09, 11←10, 12←11, 13←12, 14←10, 15←08, 16←{12,14,15}, 17←{13,16};
  - 18←{08, after 17}, 19←18, 20←19, 21←08, 22←{20,21}, 23←{22,14}, 24←{23,17}, 25←24, 26←25;
  - 27←{26,#1053}, 28←27; BM1←26, BM2←BM1, BM3←BM2; V0-V5 as in 9.3.
  - It is acyclic and identical to DESIGN 9.1/9.4. No slice needs code that only a later slice
    provides (the sub-verifiers checked each K2 and K3 forward reference).
- **K0** (00, 01): coherent. `AGENTS.md` routes `full`, as stated.
- **K1** (02-08): code is coherent.
  - P16 holds boots.
  - The SDK is back in step at 08.
  - The writer corpus, the worked session and the schema JSON are migrated in the slices that break
    them.
  - The C ABI is static.
  - But the `AGENTS.md` "approved, landing with" qualifiers on the sentences K1 lands are never
    removed (N-D).
- **K2** (09-17): **incoherent as written.**
  - After 10, a bus effect's observation binding carries a strip index `T + j`.
  - No K2 slice teaches the SDK's observation map or its selection resolver about strips (B1).
- **K3** (18-26): coherent, apart from the stale `AGENTS.md` route qualifiers (N-D).
- **C1** (27, 28): coherent. 27 keeps the follow guard, and 28 lifts it in the same batch.
- **BM** and **VCA**: coherent as drafted.

## 2a. Anchor spot-check

About 320 anchors were checked across slices 01-28, BM1-BM3 and V1-V2: about 110 by me, and 69, about
70 and about 70 by the K1, K2 and K3 sub-verifiers. Every count claim checked out exactly. Examples:

- 17 `HostPrepareCaps` literals and the 7 `..caps()` survivors;
- 4 `CompileLimits` literals;
- 17 `PreparedRoute` literals;
- 38 `GraphBuiltinsCompileRequest` literals and 21 `PreparedGraphPlanParts` literals;
- 11 `Submix {` sites and 25 `SubmixOutput` sites;
- export lists of 117 and 116;
- 41 opcodes;
- 21 route documents;
- `sessionDocumentBytes` 1905 equal to `wc -c` of `session.json`.

Slice 24's staging claim also holds: a record stages at most 2 entries, and a route command stages 1.

Five anchors are wrong. All are fixed in MINOR 4.11 or in N-J:

| Where | Cited | Actual |
|---|---|---|
| 02:35 | pan/matrix tag at `schema.rs:938-962` | module `schema.rs:950-970`; tag values `session_wire.rs:1628-1629`, `visit.rs:252-253` |
| 13:62 | `spectrum-browser-evals.mjs` `SHAPE` at `:178` | defined at `:7` (`:178` is a use) |
| 13:56-64 | the session-map stub list is complete | it misses `sdk/test/live-controls-evals.mjs:139`, `:253-261` and `:367` (N-J) |
| 18:103, :205; 20:112 | `SKILL.md:70` "lists the route keys" | `:70` lists `channel_matrix` keys; the skill has no route-key list |
| 20:61 | #1053 `:145-147` is an "extension point" | it is the A2 D1 amendment (the content is right) |

A few more are off by 1-4 lines, which is fine for a body-only implementer. For example, the inline
kind-4 composition is `lib.rs:4372`, not `:4374`, and `hasExactFields` is `:898-901`.

---

## 3. New findings

### BLOCKER

#### B1. After the K2 push, every SDK observation read throws for a session whose bus effect declares a tap

**Evidence** (checked on `main`):

- After slice 10 (D3, D5), every bus effect that declares a tap has a filed observation handle, and
  `observation_tracks[slot]` holds a **strip** index. host-web's `observation_binding`
  (`hosts/host-web/src/lib.rs:2462-2482`) reports that index as the binding's `track_index`.
- The SDK enriches the map against tracks only.
  - `enrichObservationMap(tracks, raw)` (`sdk/src/core/observation.ts:195-215`) throws
    `sdk.observation.map` when `binding.trackIndex >= tracks.length`.
  - Its callers pass `shape.tracks`: headless at `sdk/src/core/boundary.ts:622`, browser at
    `sdk/src/browser/engine.ts:605`.
- `readObservations` calls `observationMap()` first (`boundary.ts:639-641`, `engine.ts:611-612`).
  So **every** `observationMap()` and `readObservations()` call throws, reads of a track's own
  taps included, as soon as any bus console slot or insert declares a tap and observation is on.
  Bus handles are filed from slice 10, so arming is not needed.
- Selections are also resolved against tracks only (`resolveObservationAddressesWithTracks`,
  `observation.ts:231-249`, `tracks.indexOf`), so a bus effect could never be selected.
- No K2 slice authorizes `sdk/src/core/observation.ts`. Before K2, P16 meant no bus binding existed.

**Fix (slice 13, which first gives the SDK `submixes`):**

- **Authorized paths:** add `sdk/src/core/observation.ts`. `sdk/src/core/boundary.ts` and
  `sdk/src/browser/engine.ts` are already authorized.
- **Decisions:** add a **D5. Observation on strips.**
  - `enrichObservationMap` and `resolveObservationAddressesWithTracks` take the strip list
    `[...tracks, ...submixes]`, with tracks first.
  - Every caller passes `[...shape.tracks, ...shape.submixes]`.
  - `trackId` in an `ObservationMap` binding names a strip.
- **Gates:** add a gate in `sdk/test/capability-evals.mjs` (the shipped module), run by
  `check-sdk-headless.sh <A>`.
  - Use a session with a bus compressor insert, a track compressor insert and observation taps on.
  - `observationMap()` lists a binding whose `trackId` is the bus ID.
  - A `readObservations` of the **track** tap succeeds.
  - Test value: *it turns red if the SDK enriches or resolves observation bindings against tracks
    only, so one bus tap breaks every observation read.*

The alternative is slice 10 hiding bindings at strip index `>= T` until slice 13. That only defers
the same fix and leaves bus gain reduction unreadable in the SDK, so it is not recommended.

### MAJOR

Each finding names the defect, the evidence and an exact edit. "old → new" quotes the body text as it
stands in `issues/`.

#### N-A. The brace-expanded policy gates run only the first script

**Where:**

- `03:287-288`, `04:139`, `05:227-228`, `06:201-202` and `18:337-338`.
- `DESIGN.md:926-927` uses the same shorthand descriptively.

**Evidence:**

- Bash expands `bash scripts/check-{graph,realtime,workspace}-policy.sh` to
  `bash scripts/check-graph-policy.sh scripts/check-realtime-policy.sh scripts/check-workspace-policy.sh`.
- Only the first script runs. The others become positional arguments, which no policy script reads,
  so the gate is silently green.
- No `test-*` twin runs at all.
- All eleven named check/test pairs exist.

**Fix:** replace each line with an explicit loop.

- `03:287-288`. Old: `` `bash scripts/check-{graph,builtins,realtime,host-core,workspace}-policy.sh`, each with its `test-*` twin. ``
  New: `` `for x in graph builtins realtime host-core workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done` ``
- `04:139`: the same loop over `graph realtime workspace`.
- `05:227-228`: the same loop over `session protocol-control graph builtins workspace`.
- `06:201-202`: the same loop over `session protocol-control graph workspace`.
- `18:337-338`: the same loop over `session protocol-control graph builtins host-core realtime workspace`.
  Slice 18 edits builtins-compiler and host-core tests, so those two pairs are added.
- `DESIGN.md:926-927`. After the brace list, append: "This is a list, not a command: a gate runs each
  `check-*`/`test-*` pair as a separate command."

#### N-B. Slice 22's kernel-shape ratchet is vacuous, and its gate line is not a valid command

**Where:** `22:186-187` (D9), `22:211`, `22:323-325`.

**Evidence:**

- The artifact already carries **twelve** qualifying `4wide6f32x4` kernels against a minimum of 11
  (`scripts/check-web-audioworklet.sh:460-461`: "since #1110 … it carries twelve, a one-kernel
  slack").
- Raising the minimum to 12 therefore stays green even if the new kernel's `f32x4` instantiation
  fails rule 3, which is the exact #926 defect D9 exists to catch.
- "`check-web-audioworklet.sh … with --kernel-min 12`" is not a valid command either. The script
  takes `[--without-metadata-regeneration] DIR NAMED` and refuses dash arguments (`:137-141`); the
  minimum is a literal at `:471`.

**Fix:**

- **`22:186-187`.**
  - Old: "Raise `--kernel-min` from 11 to 12 at `scripts/check-web-audioworklet.sh:471`;"
  - New: "Raise the `--kernel-min` literal at `scripts/check-web-audioworklet.sh:471` to the
    qualifying-kernel count the gate reports on this slice's base plus one (13 if the base still
    carries twelve, `:460-461`), and update the ratchet comment at `:454-469`;"
- **`22:211`.**
  - Old: "(the `--kernel-min` value at `:471` only)"
  - New: "(the `--kernel-min` value at `:471` and its ratchet comment at `:454-469` only)"
- **`22:323-325`.**
  - Old: "`… simd128.named.wasm` with `--kernel-min 12`"
  - New: "`… simd128.named.wasm` (with the raised literal; record the qualifying count before and
    after)"

#### N-C. Slice 05 gate 1 cannot pass as written: the contributors pay the console limiter's latency

**Where:** `05:158-163`.

**Evidence:**

- The gate extends slice 03's "a bus equals a track fed its sum" with a session console of
  `post_insert: [true-peak limiter]`.
- Every strip carries every slot (decision 12), so session A's three **contributor tracks** also
  carry the limiter. A bypassed limiter keeps its latency (`AGENTS.md`; decision 12 L4).
- So the bus input in A arrives `L = rate/100 + 6` samples late (486 at 48 kHz). Session B's
  reference track receives the undelayed sum, and the two outputs differ by an `L`-sample shift.
  Slice 05 gate 3 itself states the 486/972 arrival times.
- `L` is not a multiple of the quantum, so the shift also changes block alignment. "Bit-identical"
  therefore fails.

**Fix:** append to `05:161`, after "the bus and the reference track carry random entries, including
random `bypass`.":

"The three contributor tracks carry every entry with `bypass: true`, so each passes its source delayed
by exactly `L = rate/100 + 6` samples (the bypassed limiter's latency). Session B's reference source
is the D3 sum delayed by the same `L` (`L` leading zeros), so both strips see the same samples at the
same frames; then the outputs are compared sample for sample."

#### N-D. Removing the `AGENTS.md` "approved, not landed" qualifiers is an orphaned deliverable

**Where:** `00:87-91` (D5), `00:111-112`; slices 03, 05, 06, 18, 20, 08 and 26.

**Evidence:**

- Slice 00 D5 writes "Approved by decision 13 …, landing with …" on every `AGENTS.md` sentence. It
  says "the umbrella's last slice to land a behaviour (named in the ruling) removes that qualifier in
  its own PR".
- No slice has that deliverable, and no slice after 00 authorizes `AGENTS.md` (checked with grep).
- So the binding agent guide would keep telling every agent that shipped features have not landed:
  the "Promising before landing" hazard, inverted.

**Fix:**

- **`00:90-91`.**
  - Old: "The umbrella's last slice to land a behaviour (named in the ruling) removes that qualifier
    in its own PR; this slice only writes it."
  - New: "Each batch's closing slice removes, in its own PR, the qualifiers of the behaviours its
    batch landed:
    - *Build submix strips and bus taps in the SDK and teach agents to author them* for the
      dual-mono strip, chain, console-slot and seven-tap sentences;
    - *Let a send follow its source strip's mute live in the browser* for the route-mute and
      follow-mute sentences;
    - the VCA batch's closing slice for the VCA sentence.

    This slice only writes them."
- **Slice 08.**
  - Add deliverable "6. `AGENTS.md`: remove the decision-13 qualifier from the K1 sentences (slice 00
    D5), changing nothing else."
  - Add the authorized path "`AGENTS.md` (those qualifiers only)".
- **Slice 26.**
  - Add deliverable "5. `AGENTS.md`: remove the decision-13 qualifier from the route-mute and
    follow-mute sentences."
  - Add the authorized path "`AGENTS.md` (those qualifiers only)".
- **CI cost.** Both are batch-closing slices whose batch already routes `full`, so this adds no CI
  run.

#### N-E. Committed tests pin base-commit values (`AGENTS.md` test rules)

**Where:** `19:174-178` (gate 4), `23:183-186` (gate 4 fallback), `23:201` (gate 6).

**Evidence:**

- `AGENTS.md` makes a one-time "no bit moved" comparison against a pre-change base PR evidence, never
  a committed test. It also allows an exact resource byte count only for a wire, ABI or on-disk
  format.
- Slice 19 gate 4 commits "`graph_metadata_bytes` equals the base commit's value … recorded in the
  test".
- Slice 23 commits "the base commit's `Some(64)` count" and "the total is the base commit's".
- The estimate is already sealed in the canonical graph text (`graph-compiler/src/canonical.rs:315`),
  which `graph_fixture -- --check` and the determinism diff hold.

**Fix:**

- **`19:174-178`.**
  - Old: "without, the estimate's `graph_metadata_bytes` equals the base commit's value for that
    session (recorded in the test with its derivation), and with, it grows by at least the bytes
    bind allocates for `RouteActivity`, measured with `bench_support::alloc`'s thread-scoped
    counters around bind."
  - New: "with the muted route, `graph_metadata_bytes` exceeds the unmuted compile's by at least the
    bytes bind allocates for `RouteActivity`, measured with `bench_support::alloc`'s thread-scoped
    counters around bind; and a test-only accessor reports that the unmuted plan builds no
    `RouteActivity`. That the unmuted estimate equals the base commit's is PR evidence (it is sealed
    in the canonical text, which gate 6 holds)."
- **`23:183-186`.**
  - Old: "If the base commit already reports different counts … (VERIFY-2 MINOR 8)."
  - New: "If the base commit already reports different counts for `None` and `Some(64)`, record both
    as PR evidence and stop for a Sol ruling; never commit a base-commit literal."
- **`23:201`.**
  - Old: "Without a depth the field is zero and the total is the base commit's."
  - New: "Without a depth the field is zero (that the total equals the base commit's is PR
    evidence)."

#### N-F. The writer-parity evals go back to `writer-evals.mjs`, and the `enginectl` test file is not authorized

**Where:**

- `18:196-197`, `18:232-233` and `18:315-317`;
- slice 20 by reference (`20:110`, `:130`, `:202`);
- `V1:205` and `V1:274`.

**Evidence:**

- `sdk/test/writer-evals.mjs` tests the `LiveControlWriter` queue contract. Its header says so, and
  slice 08 `:69` says so too. The canonical writer-parity helper is `engineCanonical()` in
  `sdk/test/console-evals.mjs:471` (VERIFY-2 M9).
- Slice 18 gate 7 and slice 20 D5 also test `enginectl`, whose eval is `sdk/test/enginectl-cli.mjs`.
  That file is not authorized.

**Fix:**

- **`18:196-197`.**
  - Old: "plus a writer-parity eval in `sdk/test/writer-evals.mjs` and a builder-default eval in
    `sdk/test/builder-evals.mjs`."
  - New: "plus a writer-parity eval in `sdk/test/console-evals.mjs` (against `engineCanonical()`,
    `:470-479`), a builder-default eval in `sdk/test/builder-evals.mjs`, and an `enginectl` case in
    `sdk/test/enginectl-cli.mjs`."
- **`18:232-233`.**
  - Old: "`sdk/test/{writer-evals,builder-evals,support}.mjs`, `sdk/test/console-evals.mjs` (only if
    a rebuild must carry `mute`)"
  - New: "`sdk/test/{builder-evals,support,console-evals,enginectl-cli}.mjs`"
- **`18:315`.**
  - Old: "In `sdk/test/writer-evals.mjs`,"
  - New: "In `sdk/test/console-evals.mjs`, against `engineCanonical()`,"
- **`18:317`.**
  - Old: "`enginectl` accepts `mute`."
  - New: "`enginectl` accepts `mute` (`sdk/test/enginectl-cli.mjs`)."
- **Slice 20** inherits the corrected list through "Exactly the authorized paths of *Mute a route in
  the session*". No edit is needed there.
- **`V1:205`.**
  - Old: "`sdk/test/{builder-evals,writer-evals,enginectl-cli}.mjs`"
  - New: "`sdk/test/{builder-evals,console-evals,enginectl-cli}.mjs`"
- **`V1:274`.**
  - Old: "including a writer-parity eval"
  - New: "including a writer-parity eval in `console-evals.mjs`"

#### N-G. Migration inventories were taken on `main`, not at the batch base, so later slices break code earlier slices add

**Where:**

- slice 06 Context (`06:42-55`) and authorized paths (`06:122-134`);
- slice 18 Context (`18:34-38`) and authorized paths (`18:219-250`);
- slice 20 by reference;
- V1 (drafted).

**Evidence:**

- **Slice 06** renames `RouteSource::SubmixOutput`.
  - Slice 03 gate 6 (extended in 05) builds its bus session through the Rust model in
    `hosts/host-web/src/tests.rs`, routing the submix to the output with that variant.
  - Slice 06 does not authorize that file, so its own workspace gate cannot compile.
- **Slice 18** makes `Route.mute` a required field.
  - Its literal and document lists are `fe8ac679`'s.
  - K1 and K2 add routed sessions in `crates/host-core/tests/submix_strip.rs` (03-06),
    `hosts/host-web/src/tests.rs` (03, 05, 10, 12, 13, 16), K2's new host-core and capi tests (for
    example `strip_handles.rs`, `strip_meters.rs`, `strip_controls.rs`, `submix_limits.rs`) and SDK
    evals.
  - None of these is authorized.
  - Slice 20 (`follows_mute`) inherits the same gap.

**Fix:**

- **Slice 06.**
  - Append to the Context list at `06:55`: "plus every site slices 03-05 added
    (`crates/host-core/tests/submix_strip.rs`, the K1 boot test in `hosts/host-web/src/tests.rs`,
    graph-compiler tests); re-run `git grep -n SubmixOutput` at the K1 branch head before editing."
  - Add the authorized path "`hosts/host-web/src/tests.rs` (the K1 boot test's route source only)".
- **Slice 18.**
  - Append to the literal list at `18:38`: "Re-run `git grep -n 'Route {' -- '*.rs'` (session
    `Route` literals) and `git grep -l '\"gain_db\"'` at the K3 base: K1 and K2 add routed sessions
    in host-core, host-web, capi and SDK tests."
  - Add the authorized path: "every file those two greps list at the K3 base, for the added key
    only".
- **Slice 20** inherits this through its authorized paths. V1 takes the same step at filing.

#### N-H. Slices 02 and 03 charge a bus's inserts in the session estimate twice

**Where:** `02:102-106` (D4) against `02:134` (deliverable), and `03:99-103` (D0).

**Evidence:**

- Slice 01 moves `estimate.rs:131-160` onto `strips()`.
- Slice 02's deliverable 1 also charges submix inserts there directly ("The estimate charges a
  submix's insert vectors and parameters"). This contradicts its own D4, which says slice 03 "charges
  them in the estimate".
- Slice 03 D0 then charges them through `strips()` and does not remove 02's loop. Every bus insert is
  counted twice, so hosts refuse sessions early.

**Fix:**

- **`02:134`:** delete the line "- The estimate charges a submix's insert vectors and parameters
  (`estimate.rs:131-160`)."
- **`03:103`.**
  - Old: "now charge bus inserts and parameters."
  - New: "now charge bus inserts and parameters, once (slice 02 adds no direct submix charge; inside
    K1 only, bus inserts are uncharged between 02 and 03)."
- **Slice 03 gate 1:** add one assertion. For a one-bus session with one insert, the session
  estimate's insert charge equals a hand count that charges that insert once.

#### N-I. Slice 08 gates 3 and 4 name an eval that renders nothing

**Where:** `08:153`, `08:198`, `08:209`.

**Evidence:** `sdk/test/headless-path-evals.mjs` is #330's shell-gate test of artifact-path bytes.
Its header says so, and it uses a fake `node` and an 8-byte wasm. It cannot boot or render a session.

**Fix:**

- **`08:153`.**
  - Old: "`sdk/test/{builder-evals,headless-path-evals,console-evals}.mjs`"
  - New: "`sdk/test/{builder-evals,console-evals}.mjs`"
- **`08:198`.**
  - Old: "(`headless-path-evals.mjs`)"
  - New: "(`console-evals.mjs`, booting and rendering through the shipped module as its
    worked-session row does)"
- **`08:209`.**
  - Old: "(`headless-path-evals.mjs`)"
  - New: "(`console-evals.mjs`)"

#### N-J. Slice 13 breaks an unauthorized SDK eval and gates a browser-only projection headless

**Where:** `13:56-67`, `13:115`, `13:145-153`, `13:155-157`.

**Evidence:**

- **The unauthorized eval.**
  - `sdk/test/live-controls-evals.mjs:253-261` is a stub `sessionMap()` reply used by
    `createBrowserLiveControls` (`:277`, `:330`); `:139` and `:367` build a `SessionMap`.
  - D3 makes the browser path copy `remoteMap.submixes` (`sdk/src/browser/live-controls.ts:48-52`),
    so these stubs throw and `check-sdk-headless.sh` goes red.
  - The file is not authorized.
- **The headless gate.** Gate 2 runs in `capability-evals.mjs`, which uses `createOfflineEngine`
  (headless, `:57`). But `MeterUpdate`, `meterProjection` and `createMeasurementFeeds` are
  browser-only (`measurement.ts:203`, `:262`, called only from `browser/engine.ts:590`), so "each bus's
  measured `peakLeft`/`peakRight`" does not exist in that eval.
- **The unobservable bullet.** Gate 3's first bullet cannot be observed before slice 17:
  `EngineLiveControls.#map` is private (`live-controls.ts:762`).

**Fix:**

- **`13:64`:** add the bullet "- `sdk/test/live-controls-evals.mjs:139`, `:253-261` (the stub reply
  `createBrowserLiveControls` reads) and `:367`;".
- **`13:62`.**
  - Old: "`SHAPE` (`:178`)"
  - New: "`SHAPE` (`:7`)"
- **`13:115`:** add `live-controls-evals.mjs` to the `sdk/test/{…}` list.
- **`13:145-153` (gate 2)**, replace with:
  - "(a) In `sdk/test/capability-evals.mjs` (headless, shipped module, run by
    `check-sdk-headless.sh <A>`), for a `T = 3`, `S = 2` session with meters on and a distinct source
    level per bus: `sessionMap().submixes` lists both IDs in canonical order, and the frame's
    `submixPeaks` pairs are ordered as that list.
  - (b) In `sdk/test/measurement-evals.mjs` (fake host), a posted 15-field `miso.meter.v1` message
    projects `MeterUpdate.submixes` keyed by ID in order. A message whose `submixCount` differs from
    the known list refuses with `sdk.meter.submix_count`. Compare structurally."
  - Keep the test-value sentence.
- **`13:155-157`:** delete gate 3's first bullet (slice 17 gate 4 covers it). Keep the
  `test-web-audioworklet.mjs` bullet.

#### N-K. `RouteActivity` is charged in neither place it must be

**Where:** `19:92-94` (D6) with `19:114` (paths); `22:180-183` (D8) with `22:135-136` (D4).

**Evidence:**

- **Slice 19 cannot reach the gates.** D6 charges `RouteActivity` "inside `graph_metadata_bytes`".
  But `resource_estimate` (`graph-compiler/src/estimate.rs:15-27`) receives no route gates; the gates
  live in the `PreparedRoute`s built in `compile.rs`. Its only caller is `compile.rs:485`, which
  slice 19 does not authorize.
- **Slice 22 adds a case no one charges.** Slice 22 D4 builds `RouteActivity` at bind for every plan
  with a live route, even with no silencing gate. A compile-time charge cannot see an attach that
  happens after compile, and D8's exact list (queues, boxes, producer table, IDs) leaves the table
  out. So gate 8's exact equality fails, or the bytes go uncharged against the #1100 rule.

**Fix:**

- **`19:114`.**
  - Old: "`crates/graph-compiler/src/estimate.rs` and `crates/graph-compiler/tests/` (one new test
    file)"
  - New: "`crates/graph-compiler/src/estimate.rs`, `crates/graph-compiler/src/compile.rs` (the
    `resource_estimate` call at `:485` only, to pass the route gates) and
    `crates/graph-compiler/tests/` (one new test file)"
- **`22:181-182`.**
  - Old: "each `Box<LiveRoute>`, the producer table, and the route IDs,"
  - New: "each `Box<LiveRoute>`, the producer table, the route IDs, and the `RouteActivity` table
    with its per-op route indices whenever the compile-time estimate did not already charge it (a
    plan whose live routes have no silencing gate),"

---

## 4. MINOR findings

1. **Slice 18 is still about a day of work.** It covers the session, wire and opcode, the graph
   coefficient interface across 17 literals, graph-compiler, the SDK, a 21-document migration with
   three derived fixtures and the pin chain, and 10 gates. The user's half-day rule is not met.
   - **Seam (recommended, not required):**
     - 18a, class A: `RouteGate`, `PreparedRoute.gate` (always `OPEN`), `gated_route_coefficients`
       replacing `folded_route`, `route_coefficients`, and the `PlanningMetadata` changes. Its gate
       is 18 gate 2 plus the determinism diff.
     - 18b: the field, wire, opcode, SDK, migration, the canonical `route-mute` row and the fold
       decline.
   - The other large slices (03, 05, 16, 22) are bounded and can be done from their bodies.
2. **Build directories.** `build-web-audioworklet.sh` refuses unless `<A>` and `<B>` exist, are empty
   and are not symlinks (`:55-64`). Any second run is refused.
   - In DESIGN 7 (`:983`), prefix the command with `rm -rf <A> <B> && mkdir -p <A> <B> &&`, as CI
     does (`qualification.yml:130-132`).
3. **The host-web allocation counter.** host-web has no `bench-support`; it installs its own global
   allocator (`ffi.rs:4463-4531`).
   - In `24:257` and `26:193`, replace "`bench_support::alloc`'s thread-scoped counters" and
     "`bench_support::alloc`" with "`crate::ffi::live_response_ffi_tests::measured` (the pattern at
     `hosts/host-web/src/tests.rs:3638-3647`)".
4. **Slice 22 gate 8 requires exact equality** with the measured bytes. Allocator rounding and
   capacity make that fragile, and the #1100 precedent asserts "at least".
   - Either assert `>=` plus equality with the stated formula, or justify exactness on the #1074
     precedent in the body.
5. **DESIGN 7 `:948-949`.** "the only CI command that runs … `console-workload`'s tests" is wrong
   for console-workload: `aarch64-release` runs it too (`qualification.yml:981`;
   `run-aarch64-tests.sh:160`). The claim holds for `tools/audit`.
6. **A superseded test survives.** Slice 10 gate 1 is a superset of the host-web K1 boot test (slice
   03 gate 6, extended by 05 gate 7).
   - Slice 10 deliverable 4 should also delete or fold that test (`AGENTS.md`: a change that
     supersedes a test deletes it).
7. **Freeze `LiveRouteState`'s constructor in slice 24.** Make it take
   `effective_mute: &dyn Fn(usize, usize) -> bool`, as `LiveRouteMuteFollow::delta` does.
   - Slice 28 seeds it in capi, which has no solo state. Its "reused, not changed" rule then holds.
8. **BM3 D4** writes its evidence paragraph into the umbrella spec. If the umbrella has closed, that
   spec has left `.github/ISSUE_SPECS/`.
   - Add the fallback: "or, if it has left, BM3's own spec and an issue comment".
9. **DESIGN 9.5 (`:1232`).** "K3's slices 18-22 touch only session, protocol, graph …" is wrong.
   Slice 18 edits `host-core/tests/{randomized,collapse_arming}.rs`, which slice 14 also edits, plus
   host-web fixtures, tools and scripts.
   - Say: "18-22 may be drafted during K2 review, but are implemented on the K2 head".
10. **#1053 landing after slice 23.** If #1053 lands after 23 with the full live-control request,
    every C ABI plan gets route lanes. The capi resource oracles and `audit capi` then move, and
    slice 23 gate 8's claim that "nothing moves in `audit capi`" no longer holds.
    - Add a conditional to slice 23, on the slice 20 D6 model: a builtins-only request, or the listed
      re-pins.
    - Add one sentence to slice 00's #1053 annotation.
11. **Anchor and wording fixes.** The section 2a table covers 02:35, 13:62 and 20:61.
    - Slice 02 should also say that `Submix` loses `#[derive(Eq)]` (`model.rs:576`).
    - In `18:103`, `18:205` and `20:112`: "add `mute` to the route key list at
      `.claude/skills/author-session/SKILL.md:70`" → "add a bullet after
      `.claude/skills/author-session/SKILL.md:70` listing a route's keys (`id, source, destination,
      channel_matrix, gain_db, mute`; slice 20 appends `follows_mute`)".
12. **Slice 04.**
    - D2 should be `NodeKind::SumDelay { line: u32, channels_agree: bool }`, because the D5 witness
      needs `channels_agree`.
    - Add `bash scripts/trace-graph-audit.sh target/release/audit` to gate 5, since the slice adds a
      render arm.
13. **Slice 05 gate 2.**
    - `Backend::Scalar` binds no banks, so restrict the `ceil(n / W)` count to
      `Backend::current()`.
    - The `plan_bank_groups` arm on hand-built candidates re-tests rack-compiler's padding, not
      graph-compiler's grouping. Feed it the compiler's console cohorts (a `test-support` accessor),
      or drop it and keep accept-or-`FOREIGN_CONSOLE_REFUSAL`.
    - Gate 6 (the rewritten generators) needs a test-value sentence.
14. **Slice 08.**
    - D1 should spell `params: []`, to match `Submix::unity`.
    - Gate 2's first bullet is self-contradictory, because the track rule refuses at `console()`
      (`session.ts:690-695`). Say "`console()` after a spec'd submix refuses with the track rule's
      message".
    - Gate 9 should add `cargo fmt`, `cargo clippy` and `RUSTDOCFLAGS='-D warnings' cargo doc` at the
      K1 head.
15. **Slice 02.**
    - Gate 7 should add `bash scripts/check-capi-abi.sh --self-test`.
    - The non-goals' red-eval list should add `sdk/test/enginectl-cli.mjs`.
16. **Slice 15 claims, in gate 1, that the host accepts reason 12, but nothing tests it.**
    - Authorize `scripts/test-web-audioworklet.mjs` and add a `{ reason: 12 }` row to its reasons
      loop (`:1820-1827`).
    - State reason 12's result code: `RESULT_INVALID_ARGUMENT` by default, `lib.rs:4301-4310`.
      Assert it in slice 16 gate 2.
17. **Slice 16: refusal reasons at a bus index.** The eq-config refusal path classifies by
    `trackIndex >= trackCount` (`worklet.js:1526-1532`, `sdk/src/core/boundary.ts:1205-1216`). So a
    bad rack or effect at a bus index reports `unknownTrack`.
    - Authorize both files, and use the strip count.
18. **Slice 10 gate 6** should add the `effect-runtime` policy pair (it polices effect-compiler) and
    the `workspace` pair.
19. **Missing test-value sentences:** slice 09 gate 3 (the new `offsetof` assertions), slice 14 gate
    3 (the new reach counter) and slice 05 gate 6.
20. **Slice 14:** `solo.rs:86` names `HostLiveControlHandles::tracks`; update it with the rename.
21. **Slice 12 D1** should say that, when `S > 0`, the message's `peaks` (`2T + 2`) is assembled from
    two fixed views, because the master is no longer adjacent to the tracks (`worklet.js:731-735`,
    `boundary.ts:1100`).
    - Slice 16 gate 5's "`submixGrDb[j]` in the message" is a JS field. The native test reads the
      frame word.
22. **Slice 18: an unauthorized test caller.** `crates/graph/src/program/tests.rs:1782` and `:2071`
    pass `BTreeMap<GraphNodeId, RouteTransform>` to `route_folds_over_program`
    (`runtime.rs:6811-6816`).
    - Keep that signature, or authorize the file.
    - Add `python3 -B scripts/check-browser-expected-resources.py --self-test`, because the slice
      edits its row at `:572`.
23. **Slice 19 D5** freezes `Option<&RouteActivity>`, but slice 22's route op writes the bit. Freeze
    `Option<&mut RouteActivity>` (or `Cell<bool>` entries) now.
24. **Slice 20.**
    - Gate 6 bullet 2 should name `crates/protocol/tests/console_session_edits.rs`, the authorized
      file.
    - Update the `InvalidEnum` doc comment ("string enum token") in `crates/session/src/diagnostic.rs`
      for the boolean use.
    - Gate 5 duplicates 18 gate 2: extend that test instead.
    - Slice 21 gate 3 overlaps gate 1's saturation case: merge them, or state what gate 3 alone
      catches.
25. **Slice 22.**
    - D7 should name the `QueueGeneration` passed to `bounded_spsc` (`spsc.rs:236-238`).
    - D3's "the last record wins, ramping from the pre-drain current" is inexact when an earlier
      record in the same drain is a step (`length == 0`). Say "each record ramps from the previous
      record's `coefficients_at(0)`".
26. **`bank_route_folds()` is plan-wide** (`runtime.rs:2347-2350`), not per bus. Slice 04 gate 3,
    slice 18 gate 4 and slice 22 gate 7 should state the expected plan count.
27. **Slice 23 D1** should say that the attach precedes the cap check (`prepare.rs:1077-1093`), so the
    charge joins `admitted_graph_and_model`.
28. **Slice 17 gate 7** (the K2 boundary) should add `cargo run --locked -p graph-compiler --bin
    graph_fixture -- --check` and `bash scripts/trace-graph-audit.sh target/release/audit`.
29. **Slice 00 files slices 01-28, the umbrella and BM1-BM3, but not its own spec.** `AGENTS.md`
    gives every local spec a GitHub issue in the same checkpoint. Say whether slice 00 is filed
    (recommended: yes, as the umbrella's first child).
30. **Slice 06 gate 1**, like N-C: say that the contributor tracks carry every console entry with
    `bypass: true` (the EQ slot is latency-free, so no shift is needed).

---

## 5. Implementability from the body alone

With the BLOCKER and MAJOR fixes applied, every slice from 00 to 26 can be implemented from its body
alone by a fresh agent:

- **00-17 and 19-26:** each is about half a day. The largest are 03, 05, 16 and 22, and each is
  bounded by its own gates.
- **Slice 18** is closer to a day (MINOR 4.1). It is the one place the user's half-day rule is
  stretched, and its seam is clean.

Without the fixes, a body-only implementer gets stranded at:

| Slice | Where it gets stuck | Finding |
|---|---|---|
| 05 | an unpassable gate | N-C |
| 06 | a compile break outside its paths | N-G |
| 08 | an eval file that cannot render | N-I |
| 13 | a red SDK eval and a gate on a browser-only projection | N-J |
| 18 and 20 | compile breaks in K1/K2 tests; parity gate in the wrong file | N-G, N-F |
| 19 | a deliverable that needs an unauthorized file; a forbidden pinned value | N-K, N-E |
| 22 | a vacuous ratchet; an uncharged table | N-B, N-K |
| 23 | forbidden pinned values | N-E |

Batch K2 would also ship B1. Slices 27-28, BM1-BM3 and V0-V5 are bounded as drafted, with their
anchors re-verified at filing as the plan already requires.

### Required edit list (for the coordinator)

| # | Finding | Files |
|---|---|---|
| 1 | B1 | slice 13 (paths, D5, one gate) |
| 2 | N-A | 03, 04, 05, 06, 18, DESIGN 7 |
| 3 | N-B | 22 |
| 4 | N-C | 05 |
| 5 | N-D | 00, 08, 26 |
| 6 | N-E | 19, 23 |
| 7 | N-F | 18, V1 |
| 8 | N-G | 06, 18 |
| 9 | N-H | 02, 03 |
| 10 | N-I | 08 |
| 11 | N-J | 13 |
| 12 | N-K | 19, 22 |

MINORs 4.1-4.30 are optional at landing. MINOR 4.11's wrong anchors are worth fixing, because
implementers read them literally.

