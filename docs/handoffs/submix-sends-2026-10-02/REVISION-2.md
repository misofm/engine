# REVISION-2: how each VERIFY-2 finding was resolved

Reviser: role "Sol briefs", second revision pass, 2026-10-02, against `main` at `fe8ac679`. Nothing in
the repository or on GitHub was changed.

**Method.** Every finding was checked against the tree before it was applied; none was taken from
`VERIFY-2.md` alone.

- **Own reads by the reviser** of every load-bearing claim: the live-control attach functions and
  their callers (`crates/effect-compiler/src/prepare.rs:1457-1641`, `crates/host-core/src/prepare.rs:860-1000`),
  host-web's effect filing, observation tables, kind-4 composition and staging count
  (`hosts/host-web/src/lib.rs:4318-4400`, `:5860-6000`, `:6253-6257`), the route coefficient path
  (`graph-compiler/src/ids.rs:268-292`, `graph/src/runtime.rs:5980-6122`, `graph/src/lib.rs:742`,
  `:2204`, `canonical.rs:255-280`) and graph's dependency gate (`scripts/check-graph-policy.sh:15-30`),
  every `PreparedRoute`, `RouteTransform`, `Route` and `EffectPreparedEntry` literal site, the ID
  staging (`lib.rs:1915-1928`, `host-core/src/shape.rs:60-82`), the meter frame readers
  (`sdk/src/core/boundary.ts:1080-1110`, `sdk/src/browser/measurement.ts:200-240`,
  `worklet-host.js:885-925`), the session diagnostic codes (`crates/session/src/diagnostic.rs`), the
  #1053 spec, the phase profile (`crates/graph/src/lib.rs:35-206`,
  `tools/console-workload/tests/gain_pan_profile.rs`), the CI commands (`qualification.yml`), the
  vocabulary and layout scripts' argument handling (each run bare), and
  `/proc/sys/kernel/perf_event_paranoid` (4).
- **Four writing passes**, each a fork of the reviser with this context, rewrote the issue bodies by
  batch against the frozen `DESIGN.md` revision 2 and re-verified every anchor they cite. Their
  corrections are folded in below (section 5).

**Outputs:**

- `DESIGN.md`, revision 2 (rewritten; section 4 unchanged);
- `issues/`, renumbered in final dependency order (00-28, BM1-BM3, V0-V5); every superseded body is
  deleted;
- this file.

Status labels: **Resolved** (fixed as asked), **Resolved differently** (fixed by an equivalent route,
with the reason), **Partly rejected** (part declined, with evidence).

---

## 1. BLOCKERs

| ID | Status | Resolution |
|---|---|---|
| N1. A `follows_mute` route into the output is silenced at preparation and never restored live | Resolved | `follows_mute` applies only to routes into a submix (DESIGN P11, 5.2, 5.6, 5.11). Slice 20 refuses `follows_mute: true` on a route into the output at `$.routes[<i>].follows_mute`, also when a `0507` edit would create it (the whole transaction refuses), and gates both. The SDK defaults `followsMute` to `true` only for a route into a submix, always writes `false` for one into the output, and refuses `true` there. Slices 26, 27 and 28 carry only "output routes never follow". BM1's fixture already writes `false` on output routes. **Code:** VERIFY-2 asked for "the existing code for an out-of-domain value". The session has no boolean-domain code (`crates/session/src/diagnostic.rs:17-73`: `numeric.out_of_schema_range` is numeric-only). The closest precedent is `schema.invalid_enum` for a closed token that is illegal in its context (`validate.rs:646-652`, a shared builtin parameter addressed other than `both`), so that is the code. No new code is allocated. |
| N2. Pushing K1 breaks every browser boot with live controls whose session has a submix carrying an effect | Resolved (option 1, placed in the effect compiler) | DESIGN P16. Slice 03 makes `attach_effect_live_controls` and `attach_effect_observation` attach only to effects whose owning strip is a track. Host-core cannot filter without changing those functions, because both walk every prepared entry (`prepare.rs:1464`, `:1585`); placing the rule there also covers any other caller. A bus effect then renders through its live-control-free path, which banks beside channelled lanes already (`control: None` entries, `prepare.rs:605`; the partial-control test at `graph-compiler/src/lib.rs:2985-3050`). K1 gate: slice 03 adds a native host-web test that boots a session whose submix carries a compressor insert with `live_control_command_queue_records = 64` and observation taps on, and renders; slice 05 extends it with a console slot. Slice 10 (K2's second slice, which also takes M6's host-web tables) removes the rule and gates the boot with every bus producer and observation handle filed. VERIFY-2 suggested slice 13 as the remover; with M6 moving the tables to the start of K2, the rule can go there. |

## 2. MAJORs

| ID | Status | Resolution |
|---|---|---|
| M1. `route_coefficients` cannot live in `crates/graph` | Resolved, in two layers | Verified: `route_transform` uses `math::db_to_gain_f32` (`ids.rs:274`) and graph's production dependencies are pinned (`check-graph-policy.sh:21-22`). `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` is the domain-checked function the compiler and every live producer call. It is built on `graph::gated_route_coefficients(&RouteTransform, RouteGate)`, a pure function that replaces the private `folded_route` and that the runtime's bind also calls, so the prepared bits (bound at bind from the unfolded transform) and the live bits cannot drift (DESIGN P11, 5.7; slices 18, 20, 22, 23, 27). host-core re-exports `RouteControlRecord` for host-web (slice 23). |
| M2. The prepared carrier for route mute and follow-zeroed columns is unspecified | Resolved | `PreparedRoute` gains `gate: RouteGate { mute, follow_zeroed }`. Slice 18 introduces the whole struct (with `follow_zeroed` always `[false; 2]`), so the `PreparedRoute` literal sites, which slice 18 lists and authorizes (builtins-compiler included), change once; slice 20 fills `follow_zeroed`. The canonical text adds `route-mute` (18) and `route-follow-zeroed` (20) rows only when set, so no existing digest moves. `route_coefficients` takes all four arguments from slice 18 on. |
| M3. Nothing keeps a live route out of the fold | Resolved | Slice 22: `PlanningMetadata::has_route_control` in both impls, `plain_route_gains` declines it, and the gate "eight banked tracks into a bus with route controls attached: `route_folds == 0` for that bus, and a gain edit applies" (DESIGN 5.7). |
| M4. host-web's kind 4 composes mute inline and ignores solo-safe | Resolved | Slice 14 makes `LiveControlSoloState::effective_mute(strip, lane)` public; slice 16 deletes the inline copy (`lib.rs:4374`, verified) and gates "under a track solo, unmuting a muted bus makes it audible, bit-identical to a host booted with the bus unmuted and the same solo" (DESIGN P7). |
| M5. `strips` is not one sorted list, but host-web binary-searches it | Resolved | Slice 10: a segment-aware lookup replaces the binary searches at `lib.rs:5078` (`resolve_observation`), `:5924` and `:5993` (verified); `live_control_tracks()` (`:2021-2022`) stays the track prefix, so slice 17's `T + j` base holds (DESIGN P9). |
| M6. K2's gates are ordered before the code they need | Resolved | host-web's per-strip `effect_base`, `rack_effects`, `observation_tracks`, `observation_present` and the strip lookup move into slice 10, with a boot gate. The bus gain-reduction gate (old 12) and the browser bus-master gate (old 11) move into slice 16, after observation commands can address a bus. Old 11's vacuous gate 1 is replaced in slice 11 by "index `T + S - 1` is accepted and echoed; `T + S` is refused with `host.observation.master_track`". Slice 10 also keeps the GR fold off strip indices `>= T` until slice 12 widens the frame. |
| M7. ID staging overflows on a long submix or route ID | Resolved | `HostSessionShape.longest_submix_id_bytes` (slice 13) and `longest_route_id_bytes` (slice 25), with the staging buffer sized from the max of all four; `crates/host-core/src/shape.rs` authorized in both. |
| M8. Authorized-path gaps | Resolved, per slice | 09: `tools/audit/src/capi.rs:264`. 12/13: the `miso.meter.v1` message shape is frozen (DESIGN P10: existing fields unchanged, `submixCount`, `submixPeaks`, `submixGrDb` appended), and the SDK and harness sites (`sdk/src/browser/live-controls.ts:48-52`, `engine.ts:590-594`, `index.ts`, the SessionMap stubs in `live-controls-types.ts`, `browser-defaults-evals.mjs`, `spectrum-browser-evals.mjs`, `check-session-map-shape.py:428`, `test-web-audioworklet.sh:326`) are authorized in slice 13, where `SessionMap` changes. 15: `scripts/fixtures/parameter-metadata-v1-self-test.json`. 16: `docs/BUILTINS_AND_METERING_V1.md:115-125`. 17: the browser path builds `SessionMap` with submixes since slice 13; slice 17 adds a browser-path eval. 18 (and 20 by reference): `randomized.rs:304-317`, `collapse_arming.rs:206`, `:216`, every session `Route {` literal, `sdk/test/support.mjs:96-99`, `expected.json:57` with `check-browser-expected-resources.py:572` (and that check in the gates), `tools/parameter-metadata/tests/abi_layout.rs` (not `src/`), `SKILL.md:70`. 20: `sdk/test/console-evals.mjs:647-660`. 25: the export list and session-map sites. 01: the `track_parameters` call at `builtins-compiler/src/lib.rs:12185`. |
| M9. Gate commands that cannot run as written | Resolved | DESIGN 7 now spells every standalone form with its argument (`check-abi-layout-v1.py` and `check-parameter-metadata-v1.py` exit 2 bare, verified), adds `cargo test --locked --release -p audit -p bench -p console-workload` (`qualification.yml:710`), `check-capi-abi.sh --self-test`, the foreign-width console rule, and the graph-determinism artifact. Slice 05 gate 2 requires no refusal only at `Backend::current()` and `Scalar` and checks foreign widths through `rack_compiler::plan_bank_groups`; slice 08 uses `engineCanonical()` and the builder writer-corpus row; slice 17 uses `insert(..)`/`console(..)`; slice 13 uses `capability-evals.mjs`'s shipped path; slices 18 and 20 refuse at `$.routes[<i>].mute` / `.follows_mute` with `schema.missing_field`; slice 26 has the browser-artifact gates; slice 01 gate 1 diffs `target/issue6/fresh-process-determinism.json` and runs `graph_fixture -- --check`. |
| M10. Slice 01's path instruction contradicts its class-A gate | Resolved | Slice 01: `StripRef::collection_path()` for the four sealed edge literals, `EffectPreparedEntry.strip_path` for the effect compiler's two diagnostics (its only production literal is `prepare.rs:596`), and the five builtins path builders named (DESIGN P1). |
| M11. #1053 coordination is incomplete | Resolved | Slice 00's annotation adds submix-strip fields (structural until slice 27) and the handle renames; slices 10 and 14 authorize `crates/capi/src/runtime/**` if #1053 has landed (DESIGN P13). The K3 writing pass found the mirror case: if #1053 lands **before** K1, no slice carried the submix-field guard. Slice 02 now does (its D8 and gate 7, conditional on #1053 having landed), slice 00's annotation names slice 02, and slice 27 lifts it. |
| M12. VCA: a prepared follow-mute ignores VCA mute | Resolved in V2 | One session helper computes each strip's effective fader and mute and feeds both `strip_parameters` and route lowering's `source_lane_muted`; `crates/graph-compiler` is authorized; new gate: a VCA-muted member's follow send is silent in a fresh plan (DESIGN 2.2a, 5.10). V3's gate 5 is then sound. |
| M13. The live-route harness hides the defects its gates claim to catch | Resolved | DESIGN 7 "Signals in live-route gates"; slices 24 (gate 1), 26 (gates 1, 4, 6) and 16 require distinct non-constant signals per track and lane, asymmetric matrices and routes to different buses. |
| M14. Slice 11's rename is churn and breaks a public SDK option | Resolved | The rename is dropped (DESIGN P17). The reviser also keeps host-core's `master_track` field name, which VERIFY-2 allowed to change, for consistency with P17 and to leave eight host-core test files alone. What remained of old 11 merged into slice 11 (host-core validation and doc) and slice 16 (browser docs at the listed sites, and the bus-master gate). |
| M15. Slices that exceed half a day, or are too thin | Resolved | Section 4 below. |
| M16. Slice 24's criterion cannot be measured on the bench host | Resolved | BM3 attributes with `graph::test_only_phase_profile` through a new ignored test modelled on `gain_pan_profile.rs`, commits no `perf.data`, runs the timed passes before committing the harness, and files no decision: the numbers go to the weekly performance pass with DESIGN 6.3's guidance (DESIGN P14). |
| M17. The benchmark wiring of 22 and 23 is incomplete | Resolved; the merge is not taken | BM1 (native row): `test-console-benchmark.sh`'s native section, a short-run record test in `tools/bench/src/console.rs` before any timing, the named sources of its facts and the explicit `console_model()` arm. BM2 (V8 document): the web-validator section of `test-console-benchmark.sh` (base record of three documents; the `'a third document'` case at `:1573` becomes a fourth; new missing-document, wrong-kind and copied-digest cases), pairwise in-run digest checks at `web-mixing-automation-benchmark.mjs:484`, `:584`, and the rounds check cited at `web-mixing-automation-lib.jq:141`. Section 4 says why the two stay separate. |
| M18. Slice 25's route ramps are steps on the C ABI | Resolved | Slice 27 uses #1053's ruled fixed ramp for every live record, per-change smoothing only where the wire carries one, extends a builtins-only live-control request to routes if #1053 rules one (A1.2), and runs gate 1 at the four launch rates (DESIGN 2.1, P6). |
| M19. VCA bodies | Resolved | V1 is implementation-ready (`sdk/test/support.mjs:96`, `tools/parameter-metadata/tests/abi_layout.rs:166`, `tests/round_trip.rs`, post-K3 counts 43 → 46, the browser document size). V2 and V3 carry M12 and the document-argument gates; V2-V5 are marked "drafted; anchors re-verified at filing". V3 was split in two (V3 admission, V4 enumeration and SDK), so the C ABI slice is now V5. |

## 3. MINORs

| # | Status | Where |
|---|---|---|
| 1 In-place inactive input | Resolved | DESIGN P4, 5.4; slice 19 (fill `+0.0`, in-place sole-contributor gate, `debug_assert!` at `bank_gather_source`). |
| 2 NaN through zero coefficients | Resolved | DESIGN 5.4 documents it. |
| 3 Unbounded domain and the ramp | Resolved | DESIGN P5, 5.7: a non-finite `target - start` applies as a step (slice 21), and `route_coefficients` refuses a non-finite folded coefficient (slice 18). |
| 4 Record and bind state | Resolved | DESIGN 5.7; slice 22 (refusal in `new`, bind state, idle-attached gate). |
| 5 Retained bind memory | Resolved | DESIGN 5.7 "Resources"; slices 19, 22, 23 (`route_control_resources`, `host.graph.resource.limit`). |
| 6 Slice 17 plumbing | Resolved | Slice 22 (template `with_builtin_banks`, no parts field, 38 literals, the `f32x4` instantiation visible to the kernel-shape gate and the `--kernel-min` bump). The kernel-shape gate moved from the kernel slice (21) to 22, because the kernel has no caller in the module until 22. |
| 7 Fresh-plan baseline | Resolved | DESIGN 7; slices 22 and 23. |
| 8 Slice 18 details | Resolved | Slice 23. |
| 9 Slices 19 and 21 | Resolved | Slice 24 does not grow `command_staging_count` (one staged record per route command fits `2 * MAXIMUM_COMMAND_RECORDS`, `lib.rs:6253-6257`); slice 26 grows it by the live-route count. `LiveRouteState` carries the source strip index (24). Slice 26's ramp length is the last strip-mute record staged for that source, in wire order. Slice 24 says its host-web anchors are pre-K2. |
| 10 Slice 00 AGENTS.md sentences | Resolved | Slice 00 phrases each as approved by decision 13 and landing with the umbrella. |
| 11 Slice 01 | Resolved | `#[non_exhaustive]` dropped; `track_count` keeps track meaning; anchors corrected. |
| 12 Slice 03 re-pins | Resolved | Slice 03 lists the sites and records fold counts; the stale "no effect racks" text is owned by slices 11 (host-core) and 16 (browser docs). |
| 13 Slice 05 corpus | Resolved | Slice 06 makes the corpus `SetRouteSource` value a tapped submix source and lists the SDK spellings red until 08. |
| 14 Slice 06 gate 3 | Resolved | Slice 07 pins the in-transaction lookup order (tracks first; DESIGN P8); the C ABI round trip is PR evidence. |
| 15 Slice 08 gate 3 | Resolved | Slice 08 says the skill gate proves command syntax only. |
| 16 Slice 24 C ABI shape | Resolved | BM3; DESIGN 6.3. |
| 17 Handoff folder | Resolved | `docs/handoffs/submix-strips-and-sends/`: slice 08 creates it with `APP-SDK.md`, slice 17 creates `APP-LIVE.md`, slice 25 adds its section. |
| 18 Slice 22 D5 counts | Resolved | BM1 keeps only plan-derived facts, each with its derivation. |
| 19 Missing test-value sentences | Resolved | Every body; DESIGN 7 states the rule. |
| 20 Missing policy twins | Resolved | Slices 09, 11, 14, 16, 17, 18. |
| 21 Slice 22 details | Resolved | BM1 (runner header, fold rationale, aggregate rule, #1107 counts, `sends_console_fixture`). |
| 22 Slice 24 preflight | Resolved | BM3. |
| 23 Slice 25 size and test values | Resolved | Split into 27 and 28; every gate has a test value. |
| 24 V2 | Partly rejected | `crates/capi/src/ffi.rs:1353` sets `EngineConfig.reserved[2]` (its `config()` at `:1155-1161` builds an `EngineConfig`, whose `reserved` stays `[u64; 4]`), not `CompileLimits`; it is untouched. V2 lists the real `CompileLimits` literal sites (`crates/capi/tests/resource_lifecycle.rs:160-187`, `crates/capi/src/runtime/tests.rs:17`, `crates/capi/src/ffi.rs:1163-1190`, `tools/audit/src/capi.rs:238-265`) and spells its gate with post-shrink indices. The browser and C ABI VCA bodies (now V3-V5) carry their test values, and V5 runs four rates. |
| 25 Wrong anchors | Resolved, one corrected | All applied. `NodeKind::LiveControlEffect` is declared at `runtime.rs:824` (VERIFY-2 said `:823`). |

## 4. Scope recommendation (VERIFY-2 section 4)

| VERIFY-2 recommendation | Done | Where |
|---|---|---|
| Merge 08 into 07 | Yes | Slice 08. |
| Drop 11's rename; merge its remainder into 10 | Yes | Slice 11 (host-core), slice 16 (browser docs and bus-master gate). |
| Merge 23 into 22 | **No** (section 5, item 7) | BM1 (native row) and BM2 (V8 document), each with its runner's `test-console-benchmark.sh` cases. |
| K4 out of the umbrella as a successor; 24 a baseline | Yes | BM1-BM3 (DESIGN 9.2, 6.3, P14). |
| 26 into DESIGN section 10 | Yes | O11; the file is deleted. |
| V0 optional; draft V1-V4 at filing | Yes, with V1 implementation-ready now | V0-V5 (V3 split, below). |
| Re-gate VCA on K3 | Yes | DESIGN 2.2a, 9.3. |
| Split 03 at D5 | Yes | 03 + 04. |
| Split 12 | Yes | 12 (frame) + 13 (names). |
| Split 13 in three | Yes | 14 (host-core) + 15 (reason vocabulary) + 16 (host-web admission). |
| Split 15 | Yes | 18 (field, wire, SDK, migration; muted = active `[+0.0; 4]`) + 19 (activity). |
| Split 17 | Yes | 21 (kernel) + 22 (render plane). |
| Keep 01 separate | Yes | 01. |

Beyond VERIFY-2, each on size evidence from the writing passes:

- old 09 is split into 09 (caps and the C ABI bound) and 10 (handles and host-web filing), because
  M6 and N2 added host-web tables, a lookup and a boot gate to an already full slice;
- old 25 is split into 27 and 28 per VERIFY-2 MINOR 23;
- the browser VCA slice is split into V3 (admission) and V4 (enumeration, SDK, metadata), as the
  route slices 24 and 25 are; the C ABI VCA slice becomes V5.

Umbrella A ends at 29 slices of more even size (VERIFY-2 estimated about 27 with its own splits),
plus BM1-BM3 outside it and V0-V5.

## 5. Findings rejected, corrected or refined

1. **N1's code** (section 1): `schema.invalid_enum`, because no boolean-domain code exists.
2. **N2's location** (section 1): the effect compiler, removed by slice 10 rather than 13. Slice 10
   also deletes slice 03's interim host-core assertion ("no bus effect gets a live channel"), which
   its own gate 2 contradicts.
3. **M14**: host-core's `master_track` keeps its name too.
4. **MINOR 24**: `ffi.rs:1353` is `EngineConfig`, not `CompileLimits`.
5. **MINOR 25**: `LiveControlEffect` is at `runtime.rs:824`.
6. **MINOR 6**: the kernel-shape gate belongs to slice 22, not the kernel slice. Also,
   `PreparedGraphPlanParts` has 21 literal sites, not 4; slice 22 adds no field to it, so none
   changes. `GraphBuiltinsCompileRequest` has 38, as VERIFY-2 said.
7. **Scope: 23 is not merged into 22.** With VERIFY-2's own M17 fixes (a short-run record test, the
   web-validator rewrite in `test-console-benchmark.sh`, pairwise in-run digests, named fact
   sources), the merged body came to about a day of tooling across two runners. The user's
   half-day sizing rule wins, so BM1 (native) and BM2 (V8) are separate; each owns its own section of
   `test-console-benchmark.sh` (`:1-1423` and `:1424-1590`), which answers M17's reason for merging.
8. **M8, slice 16's `console-evals.mjs` rebuild:** VERIFY-2 said the byte-for-byte rebuilds at
   `:681-692` go red under the SDK default. Under N1 they do not: those rebuilds cover the app and
   intended fixtures, whose routes all go to the output, where the default is `false`. Slice 20
   still fixes `rebuild()` (`:647-660`) so that a rebuild of any document with a send is exact.
9. **M18's anchor:** route keys are at `crates/session/src/visit.rs:106` (`:105` is the output key
   module); `channel_matrix` is `:109`.
10. **Anchors the writing passes corrected in DESIGN:** the sealed sidechain edge path
    `compile.rs:377` (VERIFY-2 M10 missed it; slice 01 covers it); `compile_session` sorts tracks at
    `crates/session/src/compile.rs:137-139` and submixes at `:140-142`; `folded_route`'s callers are
    `node_kind` (`runtime.rs:4063`) and `plain_route_gains` (`:6122`); `PlanningMetadata` is at
    `:6007` with impls at `:6016` and `:6089`; `host.meter.order` spans `prepare.rs:1177-1220`;
    `normalizeConsoleEntries` is `sdk/src/core/session.ts:1161-1224`.
11. **Facts the writing passes added:** `limits_are_valid` also calls `all_limits_nonzero`
    (`crates/capi/src/runtime/compile.rs:326-359`), so a zero `maximum_submixes` (slice 09) or
    `maximum_vcas` (V2) must stay out of it (DESIGN P12); the protocol has no session-root wire
    message, so V1's wire change is a `vca` message plus `0700`-`0702`; and V3 needs a reason 14
    `unknownVca` on slice 24's precedent (DESIGN 5.10, 5.11).

---

## 6. Old-to-new file map

| Revision-1 file | Revision-2 file(s) |
|---|---|
| `00-record-the-submix-send-and-vca-ruling.md` | `00-record-the-submix-send-and-vca-ruling.md` |
| `01-iterate-session-strips-not-tracks.md` | `01-iterate-session-strips-not-tracks.md` |
| `02-declare-the-submix-strip-in-the-session-grammar-and-wire.md` | `02-declare-the-submix-strip-in-the-session-grammar-and-wire.md` |
| `03-render-a-submix-strip-on-its-summed-input.md` | `03-render-a-submix-strip-on-its-summed-input.md` (lowering, P16) + `04-delay-a-submix-strips-summed-input.md` (bus delay arm) |
| `04-carry-every-console-slot-on-every-submix-strip.md` | `05-carry-every-console-slot-on-every-submix-strip.md` |
| `05-tap-a-submix-strip-at-any-send-point.md` | `06-tap-a-submix-strip-at-any-send-point.md` |
| `06-address-submix-strips-in-session-edits.md` | `07-address-submix-strips-in-session-edits.md` |
| `07-build-submix-strips-and-bus-taps-in-the-sdk.md` | `08-build-submix-strips-in-the-sdk-and-teach-agents-to-author-them.md` |
| `08-teach-agents-and-the-app-to-author-submix-strips.md` | merged into `08-build-submix-strips-in-the-sdk-and-teach-agents-to-author-them.md` |
| `09-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md` | `09-count-and-cap-submix-strips-in-host-preparation-and-the-c-abi.md` (caps, C ABI) + `10-list-every-strip-in-the-live-control-handles-and-file-bus-effects-in-the-browser.md` (handles, host-web tables, P16 removal) |
| `10-meter-any-boundary-of-a-submix-strip-in-host-core.md` | `11-meter-and-designate-submix-strips-in-host-core.md` |
| `11-designate-a-master-strip-in-host-core-and-the-browser.md` | host-core half into `11-meter-and-designate-submix-strips-in-host-core.md`; browser docs and gate into `16-address-submix-strips-in-browser-live-commands.md`; the rename is dropped |
| `12-show-and-name-submix-strips-in-the-browser-meter-frame.md` | `12-carry-submix-strips-in-the-browser-meter-frame.md` + `13-name-submix-strips-in-the-browser-session-map-and-sdk-measurement.md` |
| `13-address-submix-strips-in-browser-live-commands.md` | `14-give-every-strip-one-mute-owner-and-live-control-producers-in-host-core.md` + `15-add-the-not-soloable-command-reason.md` + `16-address-submix-strips-in-browser-live-commands.md` |
| `14-drive-submix-strips-from-the-sdk-live-controls.md` | `17-drive-submix-strips-from-the-sdk-live-controls.md` |
| `15-mute-a-route-in-the-session.md` | `18-mute-a-route-in-the-session.md` + `19-skip-an-inactive-route-in-its-destinations-sum.md` |
| `16-let-a-route-follow-its-source-strips-mute-in-the-session.md` | `20-let-a-route-into-a-submix-follow-its-source-strips-mute-in-the-session.md` |
| `17-ramp-live-send-coefficients-on-the-render-plane.md` | `21-ramp-a-sends-coefficients-with-the-indexed-ramp-kernel.md` + `22-ramp-live-send-coefficients-on-the-render-plane.md` |
| `18-produce-live-send-records-from-host-core.md` | `23-produce-live-send-records-from-host-core.md` |
| `19-admit-live-send-commands-in-the-browser.md` | `24-admit-live-send-commands-in-the-browser.md` |
| `20-enumerate-sends-and-drive-them-from-the-sdk.md` | `25-enumerate-sends-and-drive-them-from-the-sdk.md` |
| `21-let-a-send-follow-its-source-strips-mute-live-in-the-browser.md` | `26-let-a-send-follow-its-source-strips-mute-live-in-the-browser.md` |
| `22-add-a-bus-and-send-row-to-the-native-console-benchmark.md` | `BM1-add-a-bus-and-send-row-to-the-native-console-benchmark.md` (successor, outside the umbrella) |
| `23-add-the-bus-and-send-document-to-the-browser-mixing-benchmark.md` | `BM2-add-the-bus-and-send-session-to-the-browser-mixing-benchmark.md` (successor) |
| `24-record-the-bus-and-send-baseline-and-decide-on-route-fusion.md` | `BM3-record-the-bus-and-send-baseline-and-its-route-work-profile.md` (successor; baseline and phase profile; no decision filed) |
| `25-deliver-value-only-send-and-submix-strip-edits-to-the-running-c-abi-plan.md` | `27-deliver-value-only-send-and-submix-strip-edits-to-the-running-c-abi-plan.md` + `28-let-c-abi-sends-follow-their-source-strips-mute-live.md` |
| `26-deferred-bound-route-values-and-publish-their-live-metadata.md` | DESIGN section 10, O11 (file deleted) |
| `V0-vca-groups-umbrella.md` | `V0-vca-groups-umbrella.md` (filed when K3 closes) |
| `V1-declare-vca-groups-in-the-session.md` | `V1-declare-vca-groups-in-the-session.md` (implementation-ready) |
| `V2-apply-vca-offsets-and-mutes-at-preparation.md` | `V2-apply-vca-offsets-and-mutes-at-preparation.md` (M12) |
| `V3-ride-vca-groups-live-in-the-browser.md` | `V3-ride-vca-groups-live-in-the-browser.md` (admission) + `V4-enumerate-vca-groups-and-drive-them-from-the-sdk.md` (enumeration, SDK, metadata) |
| `V4-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md` | `V5-deliver-value-only-vca-edits-to-the-running-c-abi-plan.md` (after slice 28) |
