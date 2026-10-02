# VERIFY-2: second adversarial review of the submix, send and VCA plan (revision 1)

Role: Sol adversarial review (second verifier; wrote none of the plan). Reviewed `REVISION-1.md`,
`DESIGN.md` (revision 1) and every body in `issues/` (00-26, V0-V4) against `main` at `fe8ac679`
(confirmed with `git log -1`), `AGENTS.md` and the owner preference files. Nothing in the
repository or on GitHub was modified.

**Overall verdict: PASS-WITH-FIXES.**

The design is now sound. The BLOCKER-1 rule (a delayed route is never inactive) is correct for
every route shape on `main`, the indexed ramp law is well defined, activity per block is
implementable on the sequential executor, and P7 (one per-strip mute owner) is workable. Every
VERIFY-1 BLOCKER and MAJOR is resolved at the design level (section 1).

The revision introduced, or left, two new BLOCKERs and nineteen MAJORs (section 2). Every one has a
small, spec-level fix that changes no architecture: a scope line in P11, a K1/K2 boundary move, a
crate placement, an exclusion in the fold planner, a measurable attribution method for slice 24, and
many authorized-path and gate-command corrections. If the root would rather not apply about twenty
body edits itself, the alternative is a REVISION-2 by the reviser against this list, with no design
round and a diff-only re-check. **The fixes in section 2 (BLOCKER and MAJOR) are conditions of the PASS: apply them
before filing.** MINORs (section 3) may be applied while landing. The scope recommendation (section
4) is advisory but strongly recommended: it removes about five issues of ceremony.

How the evidence was gathered:

- **Own reading and code checks** of the load-bearing claims: `execute_op`'s staging
  (`crates/graph/src/runtime.rs:2970-3060`), `reduce_plane`/`reduce_many` (`:399-470`), in-place
  lowering and `is_dedicated` (`crates/graph/src/program.rs:231-238`, `:715-745`), PDC per edge
  (`crates/graph-compiler/src/pdc.rs:38-161`), route and sidechain lowering (`compile.rs:322-381`),
  `PlanningMetadata`/`plain_route_gains` (`runtime.rs:6000-6122`), `route_transform`
  (`graph-compiler/src/ids.rs:273-289`), the graph dependency gate
  (`scripts/check-graph-policy.sh:21-22`), host-core's solo state (`crates/host-core/src/solo.rs`),
  host-web's admission, effect filing and gain-reduction code (`hosts/host-web/src/lib.rs:3360-3445`,
  `:4320-4380`, `:5900-6000`), the #1053 spec, the C ABI limit pins, the commit history behind #957,
  #940, #926 and #937, and the CI router.
- **Four read-only sub-verifiers** checked every anchor, gate command, authorized path and
  test-value sentence in issues 00-08, 09-14, 15-21 and 22-26 plus V0-V4. About 600 anchors were
  checked; roughly 98 % are correct. I re-checked each of their BLOCKER/MAJOR claims that this
  document relies on, and cite the code directly.

---

## 1. VERIFY-1 BLOCKER and MAJOR findings

| ID | Status | Why |
|---|---|---|
| BLOCKER-1 (PDC-delayed send under live mute) | **RESOLVED** | Rule (a) is correct for every shape. A node with one input gets compensation 0 (`pdc.rs:63-73`), and a route node has exactly one input, so a compensation delay can sit **only** on the `RouteDestination` edge; the consumer's `InputRef.delay` (`program.rs:49-58`) is therefore a complete test of "delayed", read at bind. A route has exactly one consumer. Staging runs unconditionally for every `op.staged` entry (`runtime.rs:2996-3006`), so a delayed route that always mixes keeps its line fed with the fade and then zero-coefficient output: no cut, no stale arena words on unmute. Mid-chain taps (never in place, because the tap has a second reader), bus-to-bus routes and taps of every kind behave identically. Sidechains are direct tap-to-effect edges (`compile.rs:352-381`), not route nodes, so mute never reaches them and their own delays are untouched. Residuals are MINOR (3.1, 3.2). |
| MAJOR-1 (block in which a mute ramp ends) | **RESOLVED** | Activity is decided after the drain at the route op's start and written before the destination reads it. Units run once per block in topological order on one thread (`Runtime::execute`, `runtime.rs:2590-2700`), so the destination always runs later in the same block; no per-block prologue is needed. `k = position + f + 1` puts the snap at frame index 95 of block 3 for a 480-sample ramp at quantum 128, consistent with gate 2 of slice 17. |
| MAJOR-2 (no owner for a submix's mute) | **RESOLVED** (differently; I agree with P7) | Sizing `LiveControlSoloState` per strip with solo-safe submix entries keeps one owner, one shadow and one commit, and mute records land on `controls[strip].fader`, which the builtins loop creates per strip once slice 01 switches it. But slice 13 misses the inline copy of the composition in host-web (new MAJOR M4), and the strip list cannot be binary-searched (M5). |
| MAJOR-3 (#1053 diverges from a fresh plan) | **RESOLVED** for `follows_mute` sources and VCA members | The annotation text and the "whichever lands second" rule are right (`1053-*.md:38-40`, `:145-147` verified). Two gaps remain: submix-strip fields are not named in the annotation, and the 09/13 handle renames collide with the field names #1053's capi code will use (M11). The output-route form of the same divergence is BLOCKER N1. |
| MAJOR-4 (`route_coefficients` lacks source mutes) | **RESOLVED** in the design | The fourth argument and the "one function for prepared and live" rule are right. Two body-level defects: the function cannot live in `crates/graph` (M1), and the prepared carrier for mute and follow-zeroed columns is unspecified (M2). |
| MAJOR-5 (silence skip at `post_pan`, #940 citations) | **RESOLVED** by deferral (O2) | Agree. The corrections are recorded in O2, and `git show 5b5299f6` confirms #940 was a brief only. |
| MAJOR-6 (slices too big) | **PARTLY RESOLVED** | 02, 10 and 17 were split as asked. But 03, 12, 13, 15 and the new 17 each still exceed half a day by a wide margin, while 08, 23 and (after M14) 11 are too thin. See section 4. |
| MAJOR-7 (tier 3 committed blind; wrong seam) | **RESOLVED** | No tier is committed; slice 24 decides on a stated 5 % criterion; O3 names `FoldLane`/`fold_*` (`runtime.rs:1613-1946`) and leaves the aux seam to #210. |
| MAJOR-8 (benchmark slice; 2 % gates) | **RESOLVED** as asked (see 1.1 on the rejection) | Rewritten against the real runner (about 45 anchors in 22 verified, including `floor_row`, `underived()`, `floor_pins` and every 60-record pin); timings are descriptive. But the rewrite has new defects: 23 breaks a required CI script and 22 lacks a pre-timing record test (M17), and 24's decision cannot be measured on the bench host (M16). |
| MAJOR-9 (Simd4 coverage) | **RESOLVED** | `run-aarch64-tests.sh debug` covers capi's closure (`scripts/lib/product-crates.sh`), which includes graph, graph-compiler, host-core, lane and builtins-compiler but not host-web. One gate misuses explicit widths (slice 04 gate 2, M9). |
| MAJOR-10 (live controls are opt-in) | **RESOLVED** | Premise restated everywhere; slice 18 gate 4 replaces the vacuous gate. |
| MAJOR-11 (#957 revival without citation) | **RESOLVED** by deferral (O1) | History correction verified: `bf3bacab` deletes the fused **Output** route-fold family (#926, #937, #927) because "only a plan with no bank at all reached it"; `67649092` is #926's outlined `f32` tail; `40c62101` is #937's group of eight. |
| MAJOR-12 (anchors and authorized paths) | **RESOLVED for its listed items; the class recurs** | Confirmed: exactly 17 `HostPrepareCaps` literals; both `WebMeterHeader` reserved words in use; the reason-12 collision and its procedure; all 7 opcode-count pins and 4 hash sites. New instances of the same class are M8 and M9. |
| MAJOR-13 (K1 spellings, fixture claim, site lists) | **RESOLVED** | `pan`/`matrix` everywhere; index paths; the `Submix {` list (02) and `SubmixOutput` list (05) are complete and inside each slice's paths; three submix-declaring documents migrated in the slice that breaks them. One new contradiction in slice 01's path instruction (M10). |

### 1.1 The reviser's rejections and corrections

1. **MAJOR-8, native live-controlled row: agree, with a caveat.** Today the C ABI prepares with no
   live controls (`crates/capi/src/runtime/compile.rs:408-410`), so the static native row is a shape
   a real host runs, and the V8 row already measures the live-controlled shape on the shipped
   artifact. Adding a host-core preparation path to `console-workload` would be new benchmark
   machinery. **Caveat:** #1053 attaches live controls in capi, and slice 18 D1 then attaches route
   lanes to every C ABI plan, so after #1053 the static native row no longer matches what the C ABI
   runs. Slice 24's report must state which shape the C ABI ran at measurement time (MINOR 3.16).
2. **MAJOR-11 history correction: agree** (evidence above).
3. **MAJOR-5 tail-rule citation refinement: agree.** Rule 3 of
   `check-web-audioworklet-callgraph.py --kernel-shape` is what forces the outlined tail
   (`crates/graph/tests/MUTATIONS.md:339-340`).
4. **MINOR-10 deferred as O7: agree.** No real host session is known to carry a dangling bus.
5. **MAJOR-2 resolved differently (P7 instead of a new mirror): agree.** A second mirror would split
   ownership; the solo-safe flag is the smaller, already-tested change.
6. **MAJOR-3 both options together: agree.**
7. **New facts in REVISION-1 section 4.5: all verified.** `AGENTS.md` routes `full`
   (`python3 -B scripts/ci-path-router.py --event push --path AGENTS.md --path docs/rulings/x.md`
   prints `full`); `test-web-audioworklet.sh` is hermetic; the `COMMAND_SOLO_MODE = 12` mutation
   (`scripts/check-command-kind-vocabulary.py:395`, `scripts/test-web-audioworklet.sh:295`)
   collides with `COMMAND_INPUT_FILTERS = 12` (`hosts/host-web/src/lib.rs:862`); root field 8 is a
   gap.
8. **Revision self-fixes in 4.6: verified**, including the C ABI offsets (`abi.rs:454`,
   `abi_smoke.c:45` pin `reserved` at 176).

### 1.2 The specific design points asked about

- **The BLOCKER fix** holds for every tap, mid-chain taps, bus-to-bus sends and sidechains (table
  above). Two precision gaps: an inactive in-place single input needs an explicit `+0.0` fill (3.1),
  and a zero-coefficient mix of a non-finite `input`-tap sample is NaN on a delayed muted route but
  absent on an undelayed one (3.2).
- **Activity at block start:** sound (MAJOR-1 row). Implementation detail: the activity table must
  be passed into `execute_op` as a new borrowed argument, and an inactive **first** input must be
  "fill `+0.0`, then accumulate with `initial_store = false`"; calling `reduce_plane` on the filtered
  list would be "first active stores", which slice 15 gate 2 correctly catches.
- **The indexed ramp law:** well defined. `c(k)` is monotone in `k`; no overshoot while
  `(L - 1) * 3u < 1`, i.e. `L < 2^24 / 3 ≈ 5.59M`, so `L <= 2^22` is safe; settled bits are the
  assigned target and `mix2x2_block` unchanged, so a settled live route equals a static one. Several
  records in one drain reduce to "last record wins, ramping from the pre-drain current". Two
  MINORs: refuse a `mute = true` record with a nonzero target, and specify the bind-time state
  (3.4).
- **Solo state per strip for submix mute:** workable, with M4 and M5.
- **#1053 guard:** correct for `follows_mute` sources and VCA members; incomplete for submix fields
  and handle renames (M11); does not cover output routes (N1).
- **`follows_mute` reproducibility between a live edit and a fresh plan:** holds for routes into
  submixes. Solo is live-only (`.claude/skills/author-session/SKILL.md:70-71`), and the solo state
  is seeded from the session's fader mutes (`hosts/host-web/src/lib.rs:5877`, `:6129`), so slice
  19 D3's preparation-time `source_lane_muted` equals slice 16's prepared constant. It **fails** for
  routes into the output (N1) and for VCA-muted members (M12).

---

## 2. New findings

### BLOCKER

#### N1. A `follows_mute` route into the output is silenced at preparation and never restored live

**Affected:** DESIGN P11, 5.6, 5.7; slices 16 (D1, D4), 21 (D5), 25 (D1, D4).

**Evidence.**

- Slice 16 D1 allows `follows_mute` on **every** route, and D4 makes the SDK default `true` for new
  routes, with no destination condition.
- Routes into the output are never live (DESIGN 5.7; slice 17 D1; slice 21 D5: "Routes into the
  output follow only their prepared mute").
- So a session saved with a track muted prepares that track's main route (`post_pan` to the output,
  `follows_mute: true` by SDK default) as `[0.0; 4]` and **inactive**. In the producer's mixer, a
  kind-4 unmute raises the track's fader, but its only path to the output stays at zero until a
  plan replacement. The track cannot be unmuted live.
- The pre-fader variant is the mirror image: a live mute or solo leaves a follow-muted pre-fader
  output route open, while a fresh plan silences it, so the live render is not the committed
  model's.
- After slice 25 the C ABI has the same divergence: its D4 composes follow records only through
  `LiveRouteMuteFollow::delta`, which yields **live** routes, so a live strip unmute leaves the
  follow-zeroed output route prepared at zero. That breaks the "render equals committed model"
  contract #1053 rests on.
- A follow-muted main route also declines the master fold through `plain_route_gains`
  (`runtime.rs:6110-6122`), costing folds on a real path for no audible benefit (`post_pan` is
  already exact `+0.0` when the fader is muted).

**Required fix (one decision, applied in four places):** `follows_mute` applies only to routes into
a submix.

- Session validation refuses `follows_mute: true` on a route whose destination is the output, with
  the existing code for an out-of-domain value at `$.routes[<i>].follows_mute` (slice 16 D1, plus
  one refusal gate).
- The SDK default is `true` only when the destination is a submix; output routes always write
  `false` (slice 16 D4; P11).
- Slice 21 D5 and slice 25 D1/D4 then need no output-route clause beyond "output routes never
  follow".
- The fixtures already conform (slice 22 D1 writes `false` on output routes).

#### N2. Pushing K1 breaks every browser boot with live controls whose session has a submix carrying an effect

**Affected:** slices 01, 03, 04 (K1); 09-13 (K2); DESIGN 9.4.

**Evidence.**

- After slice 01 the effect compiler walks strips, and after slice 03 `strips()` yields submixes, so
  `prepared.entries` holds bus inserts; after slice 04 every submix carries every console slot.
- `attach_effect_live_controls` creates one producer per prepared entry
  (`crates/effect-compiler/src/prepare.rs:1457-1515`), and host-core calls it whenever a queue depth
  is set (`crates/host-core/src/prepare.rs:887-892`); observation handles likewise (`:906`).
- host-web files every producer and observation handle by binary search over `handles.tracks`
  and refuses on a miss with `web.live_controls.effects` or `.observation`
  (`hosts/host-web/src/lib.rs:5921-5927`, `:5990-5995`).
- So after the K1 push, any browser boot with live controls on (the producer's mixer sets 64) of a
  session that has a submix and any console slot or bus insert fails, until K2's slice 13 widens
  the tables. Today such a session (bare submix) boots. No K1 gate sees it, because the SDK defaults
  to no live controls (`sdk/src/core/abi.ts:186`) and the K1 host tests never boot a bus session
  with controls.

**Required fix (either; the first is smaller):**

- In slice 03, host-core attaches effect live controls and observation handles **only for
  track-owned entries** until slice 13 widens host-web (authorize `crates/host-core/src/prepare.rs`
  in 03; slice 13 removes the filter). Add a K1 gate: a host-web native test boots a session with a
  submix carrying a console slot and an insert, with `live_control_command_queue_records = 64`, and
  renders.
- Or move host-web's per-strip `effect_base`/`rack_effects`/observation tables and a strip lookup
  into K1. (M6 moves them into slice 09 for K2's own ordering; doing that in K1 instead also fixes
  this.)

### MAJOR

#### M1. `route_coefficients` cannot live in `crates/graph`

**Affected:** DESIGN P11, 5.7; slices 15 D2, 16 D2, 17, 18, 26 D2.
**Evidence.** The gain conversion is `math::db_to_gain_f32` (`graph-compiler/src/ids.rs:274`),
and graph's production dependencies are pinned to exactly `effect-contract`, `engine`, `lane` and
`rack` (`scripts/check-graph-policy.sh:21-22`; `crates/graph/Cargo.toml`). Moving the function into
graph needs a new dependency and a policy edit, neither authorized, across a deliberate boundary.
**Fix.** `pub fn route_coefficients` lives in `crates/graph-compiler` (host-core already depends on
it, `crates/host-core/Cargo.toml:25`). The render side keeps only the folded `[f32; 4]`. Replace
"`graph::route_coefficients`" by "`graph_compiler::route_coefficients`" in DESIGN and slices 15-18,
21, 25, 26; host-core re-exports `RouteControlRecord` for host-web, which has no graph dependency.

#### M2. The prepared carrier for route mute and follow-zeroed columns is unspecified

**Affected:** slices 15 D2/D6, 16 D2.
**Evidence.** `PreparedRoute { node, transform }` carries the **unfolded** `RouteTransform`
(`crates/graph/src/lib.rs:742`, `:2204`); the runtime folds it at bind (`folded_route`,
`runtime.rs:5990`), and the canonical text writes the unfolded bits (`canonical.rs:262-273`). "The
compiler calls `route_coefficients`" therefore cannot simply replace `RouteTransform`, and slice 16
adds column zeroing that the canonical text would not show (identical text, different bound bits).
builtins-compiler test literals of these types (`crates/builtins-compiler/src/lib.rs:5864`,
`:6178`, `:11170`) are forced and unauthorized.
**Fix.** Freeze: `PreparedRoute` gains `mute: bool` (slice 15) and `follow_zeroed: [bool; 2]`
(slice 16); `folded_route` applies them at bind through the same function as M1; the canonical
text adds a `route-mute` record (15) and a `route-follow-zeroed` record (16) only when set, so no
existing digest moves. Authorize the builtins-compiler literals.

#### M3. Nothing keeps a live route out of the fold

**Affected:** slice 17 D6.
**Evidence.** Fold planning asks `PlanningMetadata` (`runtime.rs:6000-6014`), whose doc requires its
exclusions to stay coupled to `node_kind` arms; `plain_route_gains` (`:6110-6122`) checks only
source, membership, binding and effect. A route-control binding is carried beside, not as, a
`GraphNodeBinding` (slice 17 D6), so it is invisible there. A bus `Input` can be a fold master
(`route_fold`, `:6351`, master clauses around `:6400-6420`), so a live route could be folded away:
never drained, its records never applied.
**Fix.** Slice 17 adds `has_route_control` to `PlanningMetadata` and both impls, and
`plain_route_gains` declines it. Gate: eight banked tracks into a bus with route controls attached:
`route_folds == 0` for that bus, and a gain edit applies.

#### M4. Slice 13: host-web's kind 4 composes mute inline and ignores solo-safe

**Affected:** slice 13 D2/D3.
**Evidence.** `hosts/host-web/src/lib.rs:4374` computes
`muted || (ready.solo.any_solo() && !ready.solo.solo(track))` itself, then `record_emitted`. With a
submix entry this stages `Mute { muted: true }` for a bus **unmute** while any track is soloed: a
bus cannot be unmuted under solo.
**Fix.** D2 adds: kind 4 takes the effective mute from the state (`effective_mute(strip, lane)`);
delete the inline copy. Gate: under a track solo, unmuting a muted bus makes it audible,
bit-identical to a host booted with the bus unmuted and the same solo.

#### M5. `strips` is not one sorted list, but host-web binary-searches it

**Affected:** slices 09 D2, 12, 13.
**Evidence.** `strips` is "tracks sorted, then submixes sorted" (slice 09 D2), while host-web
resolves IDs with `binary_search_by` at `lib.rs:5921`, `:5992` and in `resolve_observation`
(`:5076-5078`). A submix ID that sorts before a track ID is not found. Separately,
`live_control_tracks()` (`lib.rs:2021-2022`) feeds `_track_count`/`_track_id` and
`SessionMap.tracks`; it must remain `strips[..track_count]`, or slice 14's `T + j` index base
breaks.
**Fix.** Slice 09 D2: resolve by a segment-aware lookup (search tracks, then submixes) and keep
`live_control_tracks()` tracks-only; name these three sites in the body.

#### M6. K2's gates are ordered before the code they need

**Affected:** slices 09, 11, 12, 13.
**Evidence.**

- Slice 11 gate 2 and slice 12's bus gain-reduction gates need per-strip observation tables
  (`observation_tracks`, `observation_present`, `lib.rs:5986-5989`), kind 7/8 at a strip index
  (refused by the guard at `:4323-4326`), and gain-reduction slot arithmetic past `3T + 3`
  (`gain_base + track`, `:3369`, `:3415-3443`). All of that lands only in slice 13.
- Slice 11 gate 1 is vacuous in host-core: host-core has no master **reading**; it validates and
  echoes the index (`crates/host-core/src/prepare.rs:1224-1228`). The reading lives in host-web.

**Fix.** Move host-web's per-strip `effect_base`, `rack_effects`, observation tables and the strip
lookup (M5) into slice 09, with a boot gate (this is also N2's second option). Move the bus
gain-reduction gate of 12 and the bus-master gate of 11 after 13 (into 13, or into the merged
slice of section 4). Replace 11 gate 1 with "index `T + S - 1` is accepted and echoed; `T + S` is
refused".

#### M7. ID staging overflows on a long submix or route ID

**Affected:** slices 12 (submix ID export), 20 (route ID export).
**Evidence.** The ID staging buffer is sized from
`max(longest_source_id_bytes, longest_track_id_bytes)` (`hosts/host-web/src/lib.rs:1918-1925`,
`crates/host-core/src/shape.rs:62-77`). Both new exports copy into "the same ID staging buffer"; a
longer submix or route ID overruns `copy_id_into_staging` and traps the module.
**Fix.** `HostSessionShape` gains `longest_submix_id_bytes` (12) and `longest_route_id_bytes` (20),
and staging takes the max of all four; authorize `crates/host-core/src/shape.rs` in both.

#### M8. Authorized-path gaps that break the build or a gate

Each file below must be edited for the slice to compile or keep its gates green, and is not in the
slice's list.

- **09:** `tools/audit/src/capi.rs:264` (a `CompileLimits` literal, `reserved: [0; 4]`); breaks
  `cargo build -p audit` and clippy. V2 needs it too.
- **12:** `sdk/src/browser/live-controls.ts:48-52`, `sdk/src/browser/engine.ts:590-594`,
  `sdk/src/browser/index.ts`; `sdk/test/live-controls-types.ts:64-68`,
  `sdk/test/browser-defaults-evals.mjs:439`, `sdk/test/spectrum-browser-evals.mjs:349`;
  `scripts/check-session-map-shape.py:428` and `scripts/test-web-audioworklet.sh:326` (both mutate
  the exact reply field list checked at `worklet-host.js:980`). The `miso.meter.v1` postMessage
  shape (`hasExactFields`, `worklet-host.js:899-919`) must be specified.
- **13:** `scripts/fixtures/parameter-metadata-v1-self-test.json:32` (validated at
  `check-parameter-metadata-v1.py:233`, `:544-545`); `docs/BUILTINS_AND_METERING_V1.md:115-125`
  (the solo formula).
- **14:** `sdk/src/browser/live-controls.ts:48-52` (the browser path builds `SessionMap` without
  submixes, so every `edits.submix()` throws in the app; add a browser-path eval).
- **15 (and 16 by reference):**
  - `crates/host-core/tests/randomized.rs:304-317` and `collapse_arming.rs:206`, `:216` (session
    `Route {}` literals: compile break);
  - `sdk/test/support.mjs:96-99` (an inline route without `mute`: SDK headless red);
  - `hosts/host-web/tests/browser-v1/expected.json:57` (`"sessionDocumentBytes": "1905"`, the
    current size of `session.json`) and the self-test row at
    `scripts/check-browser-expected-resources.py:572`; add that check to the gates;
  - the inline route JSON is in `tools/parameter-metadata/tests/abi_layout.rs:168-174`, not
    `src/abi_layout.rs` as listed;
  - `.claude/skills/author-session/SKILL.md:70` (the route key list).
- **16:** `sdk/test/console-evals.mjs:647-660` (`rebuild()` omits `mute` and `followsMute`; with the
  SDK default the byte-for-byte rebuilds at `:681-692` go red).
- **20:** `scripts/check-abi-layout-v1.py` `EXPORTS` (`:115` onward, must equal the layout exactly,
  `:432-434`) and its self-test fixture; the stub module and session map in
  `scripts/test-web-audioworklet.mjs` (`:178-189`, `:2191-2197`); `check-session-map-shape.py:428`;
  `test-web-audioworklet.sh:326`; the SDK stubs at `console-evals.mjs:893`,
  `browser-defaults-evals.mjs:439`, `spectrum-browser-evals.mjs:349`, `measurement-evals.mjs:277`,
  `live-controls-types.ts:67`; `tools/parameter-metadata/src/abi_layout.rs:132`
  (`[&str; 116]` grows).
- **01:** `crates/builtins-compiler/src/lib.rs:12185` calls `track_parameters` directly; gate 2 says
  "no test edited". Keep `track_parameters` as a wrapper, or allow the edit.

#### M9. Gate commands that cannot run or cannot pass as written

- `python3 -B scripts/check-abi-layout-v1.py` with no argument exits 2 ("a document path or
  `--self-test` is required"; verified). Slices 11, 12, 13 and 19 use it bare. Use `--self-test`,
  then `<A>/miso-engine-v1-abi-layout.json`. Likewise `check-parameter-metadata-v1.py` needs a
  document or `--self-test` (19, 26 gate 4, V3 gate 6), and `check-command-kind-vocabulary.py`
  needs `--self-test` for its self-test (19).
- **DESIGN 7 omits** `cargo test --locked --release -p audit -p bench -p console-workload`
  (`.github/workflows/qualification.yml:710`). It is the only CI command that runs `tools/audit`'s
  tests and `console-workload`'s, so slices 15 and 16's re-pins in
  `tools/audit/src/{fixture_builtins.rs,builtins_graph.rs}` and slice 22's new test are otherwise
  never exercised. Add it to DESIGN 7 and to 15, 16 and 22.
- **Slice 04 gate 2 cannot pass on x86.** A `Backend::Simd4` compile on an AVX2 build yields
  `console.slot.unbanked`, because the EQ and compressor bank factories bind only
  `Backend::current().width()`; `crates/graph-compiler/tests/bank_levels.rs:923-941` already
  tolerates exactly that as `FOREIGN_CONSOLE_REFUSAL`. Require "no refusal" at `Backend::current()`
  and Scalar only, iterate `Backend::VECTOR` tolerating the foreign refusal, and check the
  `ceil(n / W)` count at the foreign width through `rack_compiler::plan_bank_groups`. Never return
  early on a width (`run-aarch64-tests.sh`'s no-silent-skip scan refuses it).
- **Slice 07 gate 1** names `sdk/test/writer-evals.mjs`, which tests the `LiveControlWriter` queue,
  not canonical JSON. Use `engineCanonical()` in `console-evals.mjs` (`:471-479`) and the writer
  corpus in `builder-evals.mjs`.
- **Slice 14 D3 and gate 3** call `edits.submix("drums").effect("glue")`, which does not exist:
  `effect(rack, effectIndex, effectId)` takes an index (`sdk/src/core/live-controls.ts:531-552`).
  Use `insert("glue", "miso.compressor")` or `console(slot, effectId)`.
- **Slice 12 gate 2** names `measurement-evals.mjs`, which uses only `fakeHost()`; the shipped path
  is `capability-evals.mjs:135-147`. Compare structurally, never against hard-coded rendered values.
- **Slice 15 gate 6** expects the refusal at `$.routes[<i>]`; the parser reports
  `schema.missing_field` at `$.routes[<i>].mute` (and `.follows_mute` in 16).
- **Slice 21** has no browser-artifact gate, although host-web has no aarch64 leg: add
  `build-web-audioworklet.sh`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts` and `check-sdk-headless.sh`.
- `check-graph-determinism.sh` (cited as the class-A proof in 01, 15, 17) checks only that one
  `canonical.json` graph is deterministic across fresh processes; it prints `PASS`. The fixture
  manifest comparison is `graph_fixture`'s own test. Name the artifact to diff in 01's gate 1.

#### M10. Slice 01's path instruction contradicts its class-A gate

**Evidence.** The four edge literals at `crates/graph-compiler/src/compile.rs:249`, `:287`, `:293`,
`:294` are the bare collection string `"$.tracks"`, sealed as such
(`fixtures/graph/v1/direct-route.canonical.txt:30-35`, each edge ends `\t$.tracks`). Deliverable 2
builds them from `path_prefix()`, which returns `$.tracks[id=<id>]`: every graph digest moves.
Also, some path sites have no `StripRef` in reach: effect-compiler `prepare.rs:1466` and `:1591`
format `$.tracks[id={}]` from `entry.track_id`, and builtins `parameter_diagnostic` (`:4777`),
`cutoff_path` (`:4820`) and `filter_order_path` (`:4835`) take `&Track`.
**Fix.** Add `StripRef::collection_path()` (`"$.tracks"` / `"$.submixes"`) for the four edge
literals; carry the strip's path prefix (or kind) on the prepared effect entry; name all five
functions in 01's site list.

#### M11. #1053 coordination is incomplete

- **The annotation misses submix fields.** #1053 D1 says a delta is live when "`fader` and
  `matrix_or_pan` are the only fields that differ" (`1053-*.md:38-39`), without "track". After K1 a
  submix has both fields, and #1053 pushes only track lanes. Add to slice 00's annotation: "any
  delta to a submix strip's fields is structural until *Deliver value-only send and submix-strip
  edits to the running C ABI plan* lands".
- **Handle renames collide with #1053.** Slice 09 renames `HostLiveControlHandles.tracks` to
  `strips` and slice 13 renames `track_controls` to `strip_controls`; #1053's scope keeps
  `track_controls` in capi's `ProviderEpoch` (`1053-*.md`, Scope 2). If #1053 lands first, 13 cannot
  compile inside its paths (it authorizes no capi file). Add to 09 and 13: "if #1053 has landed,
  update its capi uses (authorize `crates/capi/src/runtime/**`)", and note the rename in slice 00's
  annotation.

#### M12. VCA: a prepared follow-mute ignores VCA mute (deferred umbrella; fix at filing)

**Affected:** V2 (D2, paths), V3 gate 5.
**Evidence.** Slice 16 computes `source_lane_muted` in graph-compiler from the source strip's own
session fader mutes. V2 bakes the VCA-effective mute into the **fader** only
(`strip_parameters`), and its authorized paths exclude `crates/graph-compiler`. V3 gate 5 requires
"bit-identical to a fresh plan with the VCA muted" for a member's follow-mute pre-fader send; the
fresh plan leaks, the live path does not.
**Fix.** V2 D2: the effective mute (member OR reach) is computed once (session-level helper) and
feeds both `strip_parameters` and route lowering's `source_lane_muted`; authorize graph-compiler;
add a V2 gate (a VCA-muted member's follow send is silent in a fresh plan). V4 composes the same.

#### M13. The live-route harness hides the defects its gates claim to catch

**Affected:** slices 19 (gate 1), 21 (gates 1, 4, 6).
**Evidence.** `feed_and_render` (`hosts/host-web/src/tests.rs:2667-2684`) feeds one constant to
identical L and R planes of one shared source. Column, lane, route-index and delay-alignment errors
are then invisible, which hollows the "wrong column / wrong lane / wrong route" test-value claims.
**Fix.** Require distinct, non-constant signals per track and per lane, asymmetric matrices, and
edited routes that reach different buses, in those gates.

#### M14. Slice 11's rename is churn, and it breaks a public SDK option

**Evidence.** The wire is identical either way (boot word at offset 56, `u64`; header word at
offset 40, `u32`). Exactly 28 tracked files spell the word (verified with `git grep`; 79
occurrences): 3 generated, 4 prose, 21 hand-written. The rename changes the public SDK option
`liveControls.masterTrackPlusOne` (`sdk/src/core/abi.ts:122`) and layout names read by name, so it
forces an app migration. The plan keeps track-named words with strip meaning everywhere else: P9's
`track_index`, `builtin.meter.unknown_track`, `host.observation.master_track`, the header's
`track_count`. Tracks lead `strips`, so every existing index is unchanged.
**Fix.** Drop D2. Keep the spellings; document "strip index plus one, tracks first" at
`lib.rs:1033`, `:1118-1122`, `:1462`, both `.d.ts` at `:729`, `abi.ts:122`, `host-mirror.ts:66` and
`docs/EFFECT_OBSERVATION_V1.md`. The host-core internal field may still be renamed. What remains of
slice 11 is a few lines plus one test: merge it (section 4).

#### M15. Slices that still exceed half a day, or are too thin to stand alone

See section 4. The ones that must change before filing, because an implementer cannot finish them
in one attempt: 03 (five crates, a runtime arm, seven gates), 12 (Rust, layout and SDK), 13
(host-core producers, solo, reason vocabulary and host-web admission), 15 (session, protocol,
graph, SDK, 22-document migration and the activity table), and 17 (lane kernel plus runtime
plumbing). Each has a natural seam, listed in section 4.

#### M16. Slice 24's go/no-go criterion cannot be measured on the bench host

**Evidence.** `/proc/sys/kernel/perf_event_paranoid` is 4 on this host (checked), and the repo has
twice recorded that `perf` is unavailable for the same reason
(`docs/handoffs/plumbing-floor-2026-09-26/PLAN.md:25`;
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md:56`). D3's fallback ("not measured" means
NO-GO) therefore fixes the decision before anything runs. Bucket (a), "`mix2x2_block` under the
`Route` arm", cannot be separated by `perf` from the fold's own `mix2x2_block` calls
(`runtime.rs:1700`, `:1739`): match arms are not frames. D1's "nothing else changed" contradicts
the slice's own committed `console-workload` accessor, which `bench` links. A binary
`.perf.data` would be committed under `artifacts/`.
**Fix.** Attribute with the repo's supported tool instead: `graph::test_only_phase_profile`
(`crates/graph/src/lib.rs:55-69`, with a `ROUTE` phase and a bank fold sub-phase), driven as
`tools/console-workload/tests/gain_pan_profile.rs` drives it (#960). Or make 24 baseline-only on
the S0 (#1086) precedent and hand O1 to the weekly performance pass with the numbers. Run the timed
passes before committing any accessor; never commit `perf.data`.

#### M17. The benchmark wiring of 22 and 23 is incomplete

**Evidence.**

- **23 turns a required CI script red.** The web validator's tests live in
  `scripts/test-console-benchmark.sh:1424-1590`: the base web record has exactly two documents
  (`:1466`), and a mutation case labelled `'a third document'` (`:1573`) must be rejected. With
  `length == 3` the base record is refused. That script runs in required CI
  (`qualification.yml:1036`) and is not in 23's paths; gate 2's "if it has none" is wrong.
- **23's in-run digest checks** compare only documents 0 and 1
  (`scripts/web-mixing-automation-benchmark.mjs:484`, `:584`); the third document is never checked
  as distinct inside the run. `web-mixing-automation-validator.jq:139-142` does not exist (the file
  is 6 lines; the rounds check is `web-mixing-automation-lib.jq:141`).
- **22 has no short-run record test before the one timed run.** The precedent (#1085) added
  `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`
  (`tools/bench/src/console.rs:2840`), which runs each row briefly and asserts no render error, no
  forbidden operation and `record_validator_accepts`. 22 authorizes `console.rs` only "if a record
  field needs the new row named", so a bad `session_kind_shape` branch first shows up in 24's
  one-shot run, against `AGENTS.md`'s "preflight the schema before timing".
- **22 D5's accessors do not exist where the body says.** `console-workload` has no test-support
  feature, and `SessionRuntime` exposes no route-op or delayed-edge count. `console_model()`
  (`tools/console-workload/src/lib.rs:986-995`) needs a new arm, or the wildcard silently renders
  the intended fixture.

**Fix.** Merge 23 into 22 (they share `test-console-benchmark.sh` and the record library, and
`e1b0fed3` did both in one commit). Authorize `test-console-benchmark.sh` (base record of three
documents; rewrite `:1573`), `tools/bench/src/console.rs` (add the row to the short-run test), and
make the in-run digest checks pairwise. Name D5's sources (`GraphCompiler::evidence` and
`reductions`, `crates/graph-compiler/src/compile.rs:35`, `:65`, plus `bank_route_folds`) and list
the `console_model()` arm.

#### M18. Slice 25's route ramps are steps on the C ABI, against its own outcome

**Evidence.** Route edits carry no smoothing field on the wire (route keys 1-5,
`crates/session/src/visit.rs:105`, plus `mute` and `follows_mute`), so D2's "else 0" makes every
C ABI send mute a step: a click (#1053 A2 D3 measured -31 dB out-of-band energy for a stepped
mute). The product outcome promises "a declicked ramp", and #1053 A2 D3 recommends a fixed,
preparation-derived ramp.
**Fix.** Every live record in 25 (strip and route) uses #1053's ruled ramp length; per-change
smoothing only where the wire carries one. Also: D6 must say what happens if #1053 rules a
builtins-only live-control request (A1.2 is still open), and gate 1 should run all four launch rates
as #1053's gate 1 does.

#### M19. VCA bodies (deferred; fix at filing)

- **V1** misses `sdk/test/support.mjs:96`, a raw session with `submixes: []` and no `vcas`; with
  `vcas` required the SDK evals go red. V1 also names `tools/parameter-metadata/src/abi_layout.rs`
  where the inline session is in `tests/abi_layout.rs`.
- **V2 and V3:** M12 above.
- **V3 gate 6** runs `check-parameter-metadata-v1.py` without a document (M9).

Because every anchor will move before the VCA umbrella is filed, draft V1-V4 bodies at filing from
DESIGN 2.2a and V0's semantics, rather than filing these texts.

---

## 3. MINOR findings

1. **In-place inactive input (15).** Today's `[single] if *single == out` arm is a no-op
   (`runtime.rs:401-406`; `:568-572` for the Output). With a tap, its route and a single-input bus
   all sharing one buffer, an inactive route must make the destination `fill(+0.0)`. Say so in D3,
   and make gate 1's sole-contributor case an in-place chain (one route from `post_pan`). Route
   destinations are never bank members, so `bank_gather_source` (`:2949`) never bypasses the
   reduction; state it and `debug_assert!` it.
2. **NaN through zero coefficients (P4).** `0 * inf` is NaN. The `input` tap carries the raw
   source, so a delayed muted route can push NaN into a bus sum while an undelayed one cannot. D7
   sanitizes it at the bus input, but the bus `Input` meter sees it. Document it in P4.
3. **Unbounded domain and the ramp (17).** Until Q2, `target - start` can overflow to infinity for
   gains near +770 dB, giving a NaN step. Clamp, or refuse at the producer with `Domain`.
4. **Record and bind state (17).** Refuse `RouteControlRecord::new` with `mute = true` and a nonzero
   target. Specify the bind-time state: `start = target =` prepared coefficients,
   `position = length = 0`, `mute =` prepared mute. Add a gate: route controls attached and idle
   render bit-identically to a plan without them, including muted and delayed routes.
5. **Retained bind memory (15, 17, 18).** The activity table, per-input route indices and
   `Box<LiveRoute>` are not charged; the #1100 precedent charges every live-owner byte
   (`graph-compiler/src/estimate.rs:154-172`). Slice 18 needs a `route_control_resources` report
   field, and gate 6 should name `host.graph.resource.limit`.
6. **Slice 17 plumbing.** The template is `PreparedGraphPlan::with_builtin_banks`
   (`crates/graph/src/lib.rs:1278`), not the effect-control path; add no field to
   `PreparedGraphPlanParts` (literals at `crates/source/src/lib.rs:2406`,
   `builtins-compiler/src/lib.rs:5112`, `:6142`, `graph-compiler/tests/scale.rs:278`).
   `GraphBuiltinsCompileRequest` has 38 literals, not 39. The kernel-shape gate may not see an
   `#[inline(always)]` kernel: require the `f32x4` instantiation outlined and authorize the
   `--kernel-min` bump at `check-web-audioworklet.sh:471`.
7. **Fresh-plan baseline (17 gate 4, 18 gate 2).** "Stateless downstream" must also require an
   identity bus input section (HPF and LPF off).
8. **Slice 18.** `let artifact` at `prepare.rs:1026` needs `mut`; gate 4's fallback passes trivially
   if the base already reports 0 folds, so require a nonzero count.
9. **Slices 19 and 21.** `command_staging_count` is grown by both 19 D4 and 21 D4; pick one.
   `LiveRouteState` lacks the source strip index 21 needs. 21 D3's ramp length should read "the last
   strip-mute record staged for that source, in wire order". 19's anchors are pre-K2 line numbers;
   say so.
10. **Slice 00.** The amended `AGENTS.md` sentences describe taps on every strip, route mute,
    follow-mute and VCA before they land; mark them "approved; lands with the umbrella" so an agent
    does not assume them.
11. **Slice 01.** Drop `#[non_exhaustive]` from `StripKind`: in other crates it forces `_` arms that
    silently absorb `Submix` in slice 03 instead of failing to compile at the sites 03 must change.
    `shape.rs:79-81`'s `track_count` keeps track meaning (host-web sizes per-track shadows by it);
    only `effect_count` changes. Anchors: `track_parameters` is `:4706-4749`; the host-core track
    control requests are `prepare.rs:912-928`.
12. **Slice 03.** The re-pin list misses authorized sites: identity hashes at
    `graph-compiler/src/lib.rs:2528`, `:2569`, `:2583`; schedule and level literals at `:2416`,
    `:2455`, `:2490-2491`, `:2531`, `:2541`, `:2551-2552`; `:15112-15113`, `:4166`;
    `compile_shapes.rs:493` (`3 * 256` becomes `3 * 288`). Record fold counts for the probe's bus
    shapes before and after P1 as evidence (a lost bus fold is a cost on a real path). The stale
    "submixes carry no effect racks" text (`host-core/src/prepare.rs:307-309`,
    `docs/EFFECT_OBSERVATION_V1.md:131-133`, both `.d.ts` at `:731`) has no owner until 11.
13. **Slice 05.** The corpus route is `canonical.json`'s track route (`protocol_corpus.rs:30`,
    `:193-202`), so changing tag 2 does not move `COMPLETE_SCHEMA_HASH`, and wasm parity never sees a
    tapped submix source. Make the corpus `SetRouteSource` value a tapped submix source, then
    re-pin. List the SDK's `submix_output` spellings that stay red until 07.
14. **Slice 06.** Gate 3's test value names a defect capi cannot have (capi applies edits through
    the protocol model). Pin the lookup order inside a transaction (tracks first).
15. **Slice 08.** Gate 3 proves only command syntax: `skill.rs` substitutes `canonical-minimal.json`
    for every `.json` argument (`tools/session-validator/tests/skill.rs:68-79`). Gate 1 is the real
    proof.
16. **Slice 24 (MAJOR-8 caveat).** State which shape the C ABI ran at measurement time (static
    before #1053, live-controlled after).
17. **Handoff folder.** `docs/handoffs/submix-strips-<date>/` is created by 08, but 11 says it
    creates `APP-LIVE.md`, 14 says 11 created it, and 20 says 08 did. Fix the folder name once in 08
    and name it literally in 11, 14 and 20.
18. **Slice 22 D5.** Its counts test duplicates `check-console-fixtures.sh`'s witnesses (10 strips,
    202 routes). Keep only plan-derived facts (route ops per block, delayed edges, folds), each with
    its derivation, or drop it in favour of slice 24's census.
19. **Missing test-value sentences** on the `allocations == 0` gates (03 g7, 04 g7, 05 g6, 10 g3,
    12, 13 g7, 15 g8, 17 g6, 18 g5 and g7, 21 g4, g5, g7), and on 16 g6 and 19 g6. AGENTS.md exempts
    no new test; a count that *is* the claim needs one sentence too.
20. **Missing policy twins.** 11, 13 and 14 gates lack `test-host-core-policy.sh`; 09 and 14 lack
    `check-capi-abi.sh --self-test` (`qualification.yml:816`); 15 lacks
    `check-/test-protocol-control-policy.sh` and `sdk-package.sh check <A>`.
21. **Slice 22.** The runner header still says "the 60 records" (`run-console-benchmark.sh:20`);
    the fold rationale cites a post-pan send, but this fixture's sends are `pre_fader` and
    `post_fader` (the real reason is that `route_fold` targets only the master reduction). Add an
    aggregate rule "sends digest differs from the standing console's", modelled on the sparse triple
    (`console-benchmark-validator.jq:68-75`). State which slice re-points the record counts if open
    #1107 also adds a row. Have 22 define `sends_console_fixture` in the record library so 23 (or
    the merged slice) has a home for it.
22. **Slice 24.** Run `preflight-console-benchmark.sh --step bus-send-base` before the native run.
23. **Slice 25** is probably more than half a day; D4 (follow-mute composition and removing the P13
    guard) is a natural split. Gates 3 and 4 lack test-value sentences.
24. **V2.** `crates/capi/src/ffi.rs:1353` sets `nonzero_reserved.reserved[2] = 1`, out of bounds once
    `reserved` is `[u64; 2]`; gate 6 spells pre-shrink indices. **V3/V4:** a few gates lack
    test-value sentences, and V4 gate 1 runs two rates where #1053 runs four.
25. **Wrong anchors** (each under ±10 lines, but body-only implementers read them): `from_raw` is
    `protocol/src/model.rs:117`; `NodeKind::LiveControlEffect` is `runtime.rs:823`; `visit.rs:91` is
    `:94`; the slot-set rule is `CONTROL_PROTOCOL_REGISTRY.md:93`; `HostPrepareCaps` spans
    `prepare.rs:99-137`; `lib.rs:1053` is `:1054`; `TrackEdits` spans `live-controls.ts:423-610`;
    `normalizeConsoleEntries` ends `:1224`.

---

## 4. Scope and ceremony

**Proportion.** 25 filed slices plus an umbrella, then four VCA slices plus an umbrella, is
proportionate to the work (a grammar change, live sends on every host, VCA) **if** each slice is a
real half-day outcome. Today the plan is uneven: five slices are too big (M15) and four are too thin
or pure ceremony. Recommended reshaping, which ends at about 25 issues of more even size:

| Change | Why |
|---|---|
| **Merge 08 into 07.** | 08 is a skill paragraph, a handoff page and a one-off script, with no code and no test. It is the author-facing half of 07's SDK change and needs no verdict of its own. |
| **Shrink 11 (M14) and merge it into 10.** | Without the rename, 11 is a validation bound plus one browser test, and its browser gate must sit after 13 anyway (M6). |
| **Merge 23 into 22.** | 23 is an array entry, a jq count and one positional clause (`DOCUMENTS` grows to 3); it shares 22's fixture and has no outcome alone. |
| **File K4 (22-24) as a successor performance issue outside the umbrella, and trim 24 to a baseline.** | `AGENTS.md`: "benchmark machinery must not keep an otherwise usable product slice open when they can be stateless successor issues", and systematic optimisation belongs to the weekly performance pass. 24's census duplicates 22's facts test, its two new examples plus `REPORT.md` plus a 5 % gate are the evidence machinery the ceremony boundary warns about, and its profile cannot run here (M16). Record the baseline (the S0 precedent) and hand O1 to the performance pass with the numbers and a `test_only_phase_profile` attribution. The umbrella then closes on shipped product. |
| **Move 26 into DESIGN section 10** as a deferred item triggered by Q2. | A numbered slice file for an unfiled issue invites a spec without a GitHub issue (the `AGENTS.md` sync rule), and O1-O9 already live there. |
| **V0 optional; draft V1-V4 at filing.** | Every anchor will have moved by then (M19); V0's only gate is "children closed". |
| **Re-gate VCA on K3, not on "umbrella A closed".** | "Umbrella A closed" includes slice 25, which waits on #1053 (open, no code), and K4. V1-V3 need only K3's per-strip mute state and follow composition; only V4 needs #1053 and 25. As written, VCA is blocked by an unrelated C ABI issue. |
| **Split 03** at D5: the bus delay arm and its gate become their own K1 slice. | Five crates, a new runtime arm and seven gates (sub-verifier); the body itself plans the seam. |
| **Split 12**: 12a host-web frame, header and layout; 12b SDK session map and measurement. | Rust, layout JSON, worklet messages and three SDK modules (M8). |
| **Split 13**: 13a host-core per-strip producers and solo-safe state; 13b reason vocabulary (reason 12 and the mutation move); 13c host-web admission and bands. | Each is a full attempt on its own; the reason vocabulary alone touches nine spellings. |
| **Split 15**: 15a field, wire, opcode, SDK and migration, with a muted route as an active `[0; 4]`; 15b the P4 activity table and reduction. | No checked-in document has `mute: true`, so 15b moves no pinned bit, and the delicate runtime change gets its own verdict. |
| **Split 17**: 17a the lane kernel and the indexed ramp law (gate 1); 17b the runtime lane, drain, activity, fold exclusion (M3) and plumbing. | The kernel is independently testable and has its own browser-shape gate. |
| **Keep 01** as its own class-A slice. | It is a large refactor whose only claim is "nothing moved"; folding it into 03 would hide that claim. |

**Ceremony check against AGENTS.md's ceremony boundary.** No test greps source or prose; no digest
is pinned outside a wire, ABI or on-disk format; the class-A comparisons are PR evidence, not
committed tests. The census and profile examples in 24 are the one place tooling is added for a
decision; moving K4 out of the umbrella keeps that machinery from gating product.

**Slice 15 and 16's double migration.** Both add a required route key to the same 21 documents in
one batch. It is mechanical and `main` sees it once; no change required.

---

## 5. Implementability from the body alone

With the fixes above, every slice except 03, 12, 13, 15 and 17 (split per section 4) is
implementable from its body by a fresh agent in about half a day. Without them, a body-only
implementer is stranded at: 01 (contradictory path rule, M10), 04 (unpassable gate, M9), 09 (audit
literal, M8), 11 and 12 (gates that need 13, M6), 13 (inline composition, M4), 14 (non-existent API,
M9), 15 and 16 (compile and SDK breaks outside paths, M8; crate placement, M1), 17 (fold, M3),
20 (layout and session-map gates, M8), 22 and 23 (record test and a required CI script, M17), and 24
(an unmeasurable decision, M16).
