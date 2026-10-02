# REVISION-1: how each VERIFY-1 finding was resolved

Reviser: role "Sol briefs", revision pass, 2026-10-02, against `main` at `fe8ac679`. Nothing in the
repository or on GitHub was changed.

**Method.** Each fix was checked against the tree, not taken from either document.

- **Own reads.** The reviser read the load-bearing code directly:
  - PDC staging (`execute_op`, `StagedInput`, `DelayRef`);
  - `reduce_plane` and `reduce_many`;
  - in-place lowering (`program.rs:715-745`, `is_dedicated`);
  - `node_kind`, `arm_mono_collapse` and `UnitBanking`;
  - `route_fold`'s master clauses;
  - the D11 kernel and the fader countdown cap;
  - the compressor link masks;
  - the SDK console-entry normalisation;
  - the CI test commands.
- **Verification passes.** Four read-only passes re-verified every anchor the revision depends on:
  1. benchmark tooling;
  2. host-web and the SDK;
  3. session and compiler site lists;
  4. specs, history and CI.

  Their findings are folded into `DESIGN.md` section 3 and the issue bodies.
- **Probe.** The planner's probe was not re-run; VERIFY-1 had already reproduced it on host-core and
  the C ABI.

**Outputs:**

- `DESIGN.md`, rewritten;
- `issues/`, renumbered;
- this file.

The old bodies are superseded and deleted. The old-to-new number map is at the end.

Status labels used below:

- **Resolved**: fixed as the finding asked.
- **Resolved differently**: fixed, by a different route than the one proposed, with the reason.
- **Partly rejected**: part of the finding is declined, with evidence.

---

## 1. Delegated decisions

| Finding | Status | Resolution |
|---|---|---|
| 1(a) VCA in scope, sequenced last: UPHOLD | Resolved | VCA is a separate umbrella (`issues/V0`-`V4`), filed when the bus and send umbrella closes, with anchors re-verified then (DESIGN 2.2a, 9.2). The two weak arguments ("32 commands vs 1", "overwrites the personal mix") are withdrawn in DESIGN 2.2a. Its fix dependencies are carried: MAJOR-2 (strip-mute ownership reaches submix members via `vca_mute`, V2), MAJOR-3 (P13 guard, V2/V4) and MINOR-6 (`maximum_vcas` in `reserved[1]`, V2). |
| 1(b) The collapse-guard mechanism is misstated | Resolved | DESIGN 2.2b now names the real guard. `arm_mono_collapse` (`runtime.rs:2326-2338`) arms only where every lane ID is in the host's `eligible` set, which is `session_structural_symmetry` over tracks only (`builtins-compiler/src/lib.rs:3839-3862`, filtered at `host-core/src/prepare.rs:1284-1289`). `gathers_track_input` is a pure node-shape test that holds for bus chains. Slice 03 gate 4 adds a red mutation: putting submix IDs in the set turns the test red. |
| 1(b) Q1's premise is wrong | Resolved | Q1 is restated (DESIGN 8.2). Link is a per-bank mask splat (`crates/compressor/src/kernel.rs:330-339`, re-read by the reviser), so a per-lane link is a per-lane mask, not a bank split. The real costs are listed: the schema change, the linked path for the whole bank, and repetition in every effect with a link mode. |
| 1(b) The bus-compressor hazard is a footnote | Resolved | It is now a stated hazard in DESIGN 2.2b. Slice 08 carries it into the `author-session` skill and the app handoff: bypass the console compressor on buses, and use a linked insert for glue. |

## 2. The affine ramp law

| Amendment | Status | Resolution |
|---|---|---|
| 1. Bound `length`, saturate `position` | Resolved | `ROUTE_RAMP_LENGTH_MAXIMUM = 1 << 22`. The producer refuses a longer length with a typed error (slice 18) and the browser with the domain reason (slice 19). `position = min(position + quantum, length)` (DESIGN 5.7; slice 17 gate 1 covers 2^22 and saturation). |
| 2. Define `current(position)` exactly | Resolved | `c(0) = start`; `c(k) = Lane::fma(k, step, start)` for `1 <= k < length`; `c(k) = target` for `k >= length`; `current(position) = c(position)` (DESIGN 5.7). |
| 3. Define when `active` flips | Resolved | Activity is decided once per block, after the drain: inactive iff `mute && position >= length && undelayed`. See MAJOR-1. |
| Rename (not "affine D11") | Resolved | It is named the **indexed ramp** law and documented in a "Live routes" section of `docs/BUILTINS_AND_METERING_V1.md` (slice 17). D11 stays the master-plan law for faders and matrices (`crates/lane/src/kernels/builtins.rs:185-202`). "R1" was not used, because R-numbers are owner rulings (`docs/rulings/engine-footprint-2026-09-28.md`). |
| Justification | Resolved | DESIGN P5 now gives the verifier's reason: the coefficient is a pure function of the frame index, so any later fused or folded traversal can compute it. "Otherwise it is a scalar loop" is dropped. |

## 3. Findings

### BLOCKER

| ID | Status | Resolution |
|---|---|---|
| BLOCKER-1: live mute or unmute of a PDC-delayed send | Resolved, with rule (a) | **Rule: a route whose edge carries a compensation delay is never inactive.** Muted, it mixes its zero target into its own buffer, and the consumer keeps staging it. The fade therefore reaches the bus whole, `d` samples later and aligned, and the line then holds only zero-coefficient output, so an unmute can never release stale audio. Prepared semantics match: a session-muted delayed route is active with `[0; 4]` (slice 15). |

Why (a) over (b), deferred deactivation:

- It adds no state and no per-block countdown.
- It does not touch #899's staging copy (#899 is the owner of that copy).
- Its cost is exactly today's work for a muted delayed send.

Option (b) is deferred item O6, to be taken only on a measurement.

Two regression gates guard it:

- **Slice 17 gate 3.** Mute (480), idle 10 blocks, unmute (480) on a send with a 486-sample
  compensation delay. Every bus-input sample over the round trip must equal a scalar oracle (`c(k)`
  applied to the tap, delayed 486, summed in D9 order). From the unmute-ramp end plus 486 samples it
  must be bit-identical to a fresh plan. The red mutation "a delayed muted route goes inactive" is
  recorded.
- **Slice 15 gate 4.** A muted delayed route stays active, and its op runs.

Where it is stated: DESIGN P4, 5.4 and 5.7; slices 15, 17 and 21 (gate 6 for follow-mute).

### MAJOR

| ID | Status | Resolution |
|---|---|---|
| MAJOR-1: the block in which a mute ramp ends must be mixed | Resolved | Activity is decided at block start after the drain, and is constant for the block (DESIGN 5.4, 5.7). Slice 17 gate 2: a 480-sample ramp at quantum 128 reaches `k = 480` at frame index 95 of the fourth block. That block is bit-identical to the oracle (mixed whole), and the route is inactive from the next block. |
| MAJOR-2: no control-plane owner for a submix's mute | Resolved differently | The verifier proposed a new `{user_mute, vca_mute}` mirror. Instead, **P7 sizes the existing `LiveControlSoloState` per strip, with solo-safe submix entries**: never soloable, never solo-muted. Kind 4 on any strip goes through it (`hosts/host-web/src/lib.rs:4349-4380`), and kind 9 at a submix refuses with the new reason `notSoloable`. One owner, one shadow, one commit, reusing tested code. VCA adds `vca_mute` to the same state (V2). Follow-mute reads its effective mute for tracks and submixes (slice 21). Gates: slice 13 gates 2 and 3, slice 21 gate 5, V2 (a VCA mutes a submix member). |
| MAJOR-3: #1053 diverges from a fresh plan after 10 or 21 | Resolved | P13 takes the verifier's two options together, so whichever lands second carries the guard. Slice 00 annotates #1053's spec at its A2 D1 "Add:" bullet (`1053-*.md:145-147`), phrased on the committed model because D1 inspects no opcodes (`:38-40`). If #1053 is already on `main`, slice 16 (follow-mute sources) and V2 (VCA members) implement the guard in its classifier, with a capi gate. Slice 25 and V4 remove the guard by composing the dependants. |
| MAJOR-4: `route_coefficients` cannot reproduce a prepared `follows_mute` route | Resolved, with both options | `follows_mute` moved to its own slice (16). The shared function gains `source_lane_muted: [bool; 2]`: `route_coefficients(gain_db, matrix, mute, source_lane_muted)`, and the prepared and live paths both call it. The host-core mirror carries the prepared source mutes (slice 19), and slice 21 updates them live. Gate: slice 16 gate 5 (the function equals the compiler's bound constant for random values and mutes), plus slice 18 gate 1. |
| MAJOR-5: issue 16's settled-matrix condition and #940 citations | Resolved by deferral, with the corrections recorded | The silence-skip slice is deferred item O2, waiting on the silence architecture (A0/S10) and #1072. It is not filed. Its DESIGN entry carries every correction: (1) at `post_pan`, the fader must be settled-muted **and** the matrix settled for the whole block; (2) #940's original mutation list (including "swap the two inputs within a pair"), from `git show 5b5299f6`; (3) kernel-shape rule 3 of `check-web-audioworklet.sh`; (4) #1072 is interacting; (5) counts derived from the plan. The solo saving it targeted is now delivered without new machinery: a follow-muted send is inactive (P4), and the SDK defaults `followsMute` to `true` (P11). |
| MAJOR-6: slices too big | Resolved | Old 02 is now 02 (grammar and wire) plus 03 (lowering). Old 10 is now 15 (mute), 16 (follow-mute) and 26 (domains, owner-gated). Old 17 is deferred O3, which names its own split. Splits beyond the finding: old 06 into 07 (SDK code) and 08 (skill and handoff); old 07 into 09, 10 and 11; old 08 into 11 and 12; old 09 into 13 and 14; old 11 into 17 (render plane) and 18 (host-core producer); old 12 into 19 and 20; old 14 into 22, 23 and 24. Every slice is about half a day with one outcome. |
| MAJOR-7: tier 3 committed before measured, on the wrong seam | Resolved | No tier is committed (P14). Slice 24 measures a real bus-and-send session and files route fusion only if route ops plus route-input reductions reach at least 5 % of the native row's profile samples (DESIGN 6.3, with the threshold's reason). Fold generalisation is deferred O3/O4, triggered only after O1 lands and a re-measurement shows a residual of at least 5 %. O3 names `FoldLane`/`fold_plane`/`fold_cohort`/`fold_resident_tiles` (`runtime.rs:1613-1946`) as its template and leaves the aux seam to #210. Issue 19's padding and order analysis folds into slice 24 (padding) and Q5 (order). |
| MAJOR-8: benchmark slice mis-specified; "2 % slower" gates | Resolved; part rejected (see section 4) | Rewritten against the real runner as 22/23/24. Slice 22 commits a **derived fixture** (`console-sixty-four-track-sends.json` plus `scripts/derive-sends-console-fixture.py`, held by `check-console-fixtures.sh`, on the `e1b0fed3` precedent) and authorizes `floor.rs` (`floor_row` exhaustive `:154-263`, `underived()` `:370-375`), the jq `floor_pins` (`:93-170`), every 60-record pin (`run-console-benchmark.sh:381`, `console-benchmark-validator.jq:10,15,16-27,30`, preflight `:84`), `test-console-benchmark.sh`'s positional mutations, and the strip-rows-last test (`lib.rs:3096`). It uses `preflight-console-benchmark.sh --step NAME` and keeps the row out of `every_standing_workload_folds_one_route_per_track`. Preparation is stated honestly: the compilers host-core calls, not host-core, which is the C ABI's static shape. Slice 23 adds the document to the V8 benchmark, which boots through host-core **with** live controls (`web-mixing-automation-benchmark.mjs:306,323`). Slice 24 records the baseline in `artifacts/steps/bus-send-base/` (S0 and S4 live in `artifacts/steps/console-strip-{base,after}/` at HEAD). Every timing is **descriptive**: no slice passes or fails on "≤ 2 % slower". |
| MAJOR-9: Simd4 and bank-coverage gates cannot run | Resolved | DESIGN 7 and every slice name `bash scripts/run-aarch64-tests.sh debug` (arm64, or CI `aarch64-debug` at the batch push). It covers 25 product crates including graph, graph-compiler, host-core, lane and capi, but not host-web. Explicit `Backend::Simd4` iteration also runs on x86, because `Simd4` is unconditional (`crates/lane/src/backend.rs:30-37`); slice 04 gate 2 and slice 17 gate 1 use that. `run-wasm-gates.sh` is no longer cited for crate tests. host-web's 4-lane coverage is the shipped module through `check-sdk-headless.sh` and `check-browser-expected-resources.py --artifacts`. Fold-reach requirements went with the deferred fold tiers. Slice 04 adds a submix-console reach counter to host-core's `randomized.rs`. |
| MAJOR-10: live controls are not on by default | Resolved | DESIGN 3.1 and 5.7 state it: the default is 0 (`hosts/host-web/src/lib.rs:1102-1104,1137`; `sdk/src/core/abi.ts:186`), the producer's mixer opts in with 64, and so does the V8 benchmark. Old issue 11's gate 6 is replaced by slice 18 gate 4: the standing intended fixture prepared through host-core with and without a control depth has identical fold counts and bits. Tier 3's value was re-weighed and deferred (R9, O4). |
| MAJOR-11: issue 15 revives a #957-deleted family without citation | Resolved by deferral, with citation | Fusion is deferred O1, filed only on slice 24's go. Its entry cites #957 (`bf3bacab`), which deleted the fused Output route-fold family (#926, #937, #927) because "only a plan with no bank at all reached it". It names the real host path now: every mid-chain-tap send and every live send is an unfolded route op plus a reduction on every host. It carries the #926 outlined `f32` tail (`67649092`) and #937 group-of-eight (`40c62101`) lessons. |
| MAJOR-12: anchor and authorized-path errors | Resolved | Per body. Highlights: all 17 `HostPrepareCaps` literals (slice 09); the 72-byte meter header (both reserved words are in use) with every pin of 64 and every `3T + 3` spelling, which now also includes the host `.d.ts:101-106`, its SDK mirror, `test-web-audioworklet.mjs`, `direct-oracle.mjs`, `capability-evals.mjs`, `measurement-evals.mjs` and the qualification harness (slice 12); all 28 `master_track_plus_one` files (slice 11); the reason-12 collision and its fix procedure (slice 13); the opcode-count pins including `fuzz/corpus/complete-schema-manifest.md` and the schema-hash literal's four sites (DESIGN 5.11; slices 15, 16); `parameter-metadata --check <dir>` (slice 26). |
| MAJOR-13: K1 spellings, fixture claim, site lists | Resolved | `pan` or `matrix` everywhere (DESIGN 5.2; slices 02, 07). Session validation paths are by index (slices 02, 04, 05). The three submix-declaring documents are migrated in the slice that breaks them: the writer corpus (regenerated), `worked-session.json`, and `invalid-scc-diagnostics.json` (kept as a graph-level literal, slice 03). The full `Submix {` and `SubmixOutput` site lists are in slices 02 and 05. Missed sites added: the builtins seal and `processor_seal`, `planned_strip_banks`, `effect_path` and the sealed edge paths, `shape.rs`, the session estimate (slice 01), `reduction_records` and the graph-compiler estimate (slice 03), the enginectl CLI and SDK tests (slice 07), and the sidechain sharing `route_source::KNOWN` (slice 05, with a tapless-sidechain refusal test; refusals live in the session and wire tests, because the all-opcode conformance corpus holds one successful edit per opcode). |

### MINOR

| ID | Status | Resolution |
|---|---|---|
| 1. Issue 10 gate 1 overclaims | Resolved | Slice 15 gate 1 compares against an independent P4 oracle, not "the session without the route". |
| 2. Issue 15 gate 3 (arena buffer count) | Resolved in the deferred text | O1 counts retired route ops, because an in-place route owns no buffer (`program.rs:715-745`, re-read). |
| 3. Issue 16 gate 4 hard-codes 112 | Resolved in the deferred text | O2: counts are derived from the compiled plan. |
| 4. `AGENTS.md` dual-mono sentence | Resolved | Slice 00 changes it to "Strips (tracks and submixes) are dual-mono". |
| 5. "Unity" console entries are not unity | Resolved | P15: `Submix::unity(id, console)` and spec-less `submix(id)` use `bypass: true` entries, with a latency note (slices 04, 07). The app handoff covers migrating saved documents (slice 08). |
| 6. C ABI caps derived from `maximum_tracks` | Resolved | P12: `compile_limits.reserved[0]` becomes `maximum_submixes`, with 0 meaning `maximum_tracks`. Size 208 is unchanged. `limits_are_valid` (`compile.rs:354-358`) accepts it. `maximum_vcas` takes `reserved[1]` in V2. Slice 09 gate: a 2-track, 3-bus session prepares. |
| 7. Latency grows with bus depth | Resolved | Owner question Q3, with a recommendation (DESIGN 8.2); nothing filed depends on it. |
| 8. Solo leaks through pre-fader sends by default | Resolved | P11: the SDK defaults `followsMute: true`, as SSL does. The owner is told (Q4, informational). |
| 9. Issue 13's batch stated two ways | Resolved | Slice 25 is "after #1053" in both its header and DESIGN 9.4. |
| 10. Dangling buses computed every block | Resolved differently | Recorded as deferred O7, not filed now. Its trigger is the next silence slice or a real session that has one, because no real host session is known to carry a dangling bus and the owner wants no optimisation without a real path. |
| 11. Design vs issue 11 mechanism | Resolved | DESIGN 5.7: `NodeKind::LiveRoute` plus `GraphRouteControlBinding`, explicitly not a `GraphNodeBinding`. |
| 12. `master_strip` does not retire the stopgap | Resolved | DESIGN 5.8 and slice 11: it widens the "designation, not discovery" stopgap. |
| 13. Issue 22 zero direction; slice reference | Resolved | V3: a redundant mute retarget moves a settled `+0.0` to `-0.0` (`host-core/src/solo.rs:32-41`). Kind numbers are "the next free at implementation (16 and 17 if slice 19 took 13-15)". |
| 14. #1054's optional `controlSmoothing` | Resolved | DESIGN R5. The re-verification also found that #1054's keys are camelCase while the schema is snake_case, and that it claims no field ID. P6 depends only on its table, so per-change smoothing is carried until #1054 settles its own shape. |
| 15. Issue 07's meter-order wording | Resolved | Slice 10 restates `host.meter.order` from the code. |

### Section 4.3 and 4.4 imprecisions

| Item | Resolution |
|---|---|
| `kernels/builtins.rs:282` is a signature | The arithmetic is at `:282-305` (`matrix2x2_block_without_identity`). Only the deferred O2 text cites it. |
| `checked_fader_gain` vs `prepare_sections` | The prepared range check is `crates/builtins/src/lib.rs:3131-3146` (builtins, not builtins-compiler). Session validation checks `fader_db` for finiteness only (`validate.rs:351-356`), so V1's VCA range check is new session validation. |
| `capi_retained_bytes` is a field | Slice 25: the function is `capi_resources` (`compile.rs:109-213`); the field is `abi.rs:286`. |
| `model.rs:687-708` are apply arms | Slices 06 and 15 cite the enum span `model.rs:20-106` and the apply arms separately. |
| 19 vs 20 route documents | Re-counted at **21 files plus the embedded writer-corpus document** (slice 15 lists them). VERIFY-1's 20 was also short. |
| `MANIFEST.tsv` pins `canonical.json` transitively | DESIGN R4 gives the full chain: the toml `:12` files, `fixture_builtins.rs:5102`, `MANIFEST.tsv:10-11`, `builtins_graph.rs:52`, `fixture_builtins.rs:5275`, and the generated `fixtures/graph/v1/direct-route.*`. |
| `worklet.js:951` | Cited as the `miso.sessionmap.v1` reply (slices 12, 20). |
| `:393, :432-449` are in `check-command-kind-vocabulary.py` | Slice 19 cites them there and re-anchors the self-test literal sets. |
| Opcode counts assume #1054 adds none | Confirmed: #1054 claims no opcode. The counts are now 41 → 42 → 43, then +3 for VCA. |
| `from_session` at `:3924`; `:3350` is the preflight; `:3327-3345` also validates meters | Slice 01 uses the corrected anchors. |
| SDK constant is `OBJECT_KEY_ORDERS` | Slice 07 (`session-json.ts:67-88`). |
| `graph/src/lib.rs:1591-1596` allows `TrackStage` **or** Output | DESIGN 5.8. |
| `folded_bus_parts` is `#[cfg(test)]` | Confirmed by the reviser (inside `mod tests` from `runtime.rs:6881`). P1's argument from it is removed. |
| Issue 05 D3 (`rack_mut` vs `knobs_mut`) | Slice 06: the structural rack edits refuse console racks, while `020a`, `020d` and `020e` edit through `knobs_mut`. |
| "the only way to change a route is `0505`" | DESIGN 3.1: all of `0500`-`0505` are structural. |
| "-inf refused by `route_transform`" | DESIGN 3.1: session validation refuses non-finite values first (`validate.rs:481-491`). |
| "both launch-rate pairs used by `randomized.rs`" | Slice 03: one rate per seed from the launch rates, at quantum 128 (`randomized.rs:57`, `:328`). |
| D7 sanitization on bus sums | DESIGN 5.4 states it alongside the `-0.0` case. |
| Issue 05 gate 3 citation | Slice 06 cites `docs/CONTROL_PROTOCOL_SEMANTICS.md:17`. |
| Conformance feature set | DESIGN 7 uses CI's `test-debug-b` command. |
| `run-aarch64-tests.sh` argument | `debug` everywhere. |

---

## 4. Findings or evidence partly rejected or corrected

1. **MAJOR-8's "add live-control variants of the send rows" for the native benchmark is declined.**
   - The live-controlled shape is already measured on a real path: the V8 mixing benchmark boots the
     shipped artifact through host-core with live controls (64 queue records). Slice 23 adds the
     bus-and-send document there.
   - A native live-controlled row would need a host-core dependency in `console-workload` and a new
     preparation path in the benchmark, which is new benchmark machinery the owner did not ask for.
   - The native static row is not "a path production does not run": a C ABI fan-playback plan (no
     live controls until #1053) is exactly that shape.
2. **MAJOR-11's history is corrected**, from `git log` and `git show bf3bacab`:
   - #957 deleted the fused **Output** route-fold family: #926, #937, and #927's in-place source
     read.
   - #940 was a brief only (`5b5299f6`) and was never implemented.
   - The outlined `f32` tail lesson is #926's, not #920's.
3. **MAJOR-5's tail-rule citation is refined.** `check-web-audioworklet-callgraph.py` has no explicit
   "non-generic `#[inline(never)]` tail" rule. Its kernel-shape rule 3 (every `4wide6f32x4` function
   with `f32x4` arithmetic must have more vector than scalar instructions) is what forces the
   outlined tail (`crates/graph/tests/MUTATIONS.md:339-340`).
4. **MINOR-10 is not filed** (deferred O7). The reason is in the table above.
5. **New facts the review did not raise**, now handled:
   - `AGENTS.md` is not an "evidence" path for `scripts/ci-path-router.py`, so slice 00 runs the Rust
     jobs (old issue 00 gate 3 was wrong).
   - `test-web-audioworklet.sh` is a hermetic stubbed harness, so the old "under V8 through
     `test-web-audioworklet.sh`" gates (old 12, 22) could never render the shipped module. Real-wasm
     gates now use `check-sdk-headless.sh` or `check-browser-expected-resources.py --artifacts`.
   - The command-kind vocabulary's existing `COMMAND_SOLO_MODE = 12` drift mutation already collides
     with `INPUT_FILTERS = 12`. Slice 19 moves it.
   - Root field ID 8 is an unrecorded gap from the removed `limits` key. V1 records it and never
     reuses it.
   - Every gate that compares a live edit with a "fresh plan" now requires a stateless downstream or
     a paired comparison (DESIGN 7). A stateful effect after the edited value can keep the two
     histories apart for good.

6. **Defects the reviser found in the revision's own drafts, and fixed before hand-off:**
   - **Slice 02/03 ordering.** A draft added `StripKind::Submix` in the grammar slice. Every compiler
     loop slice 01 routes through `strips()` would then have walked bus strips before they could be
     lowered. The variant, the submix iteration and the estimate charge now land in slice 03, with
     the lowering.
   - **Slice 17 gate 3's oracle.** It summed with a contributor that runs through a stateful limiter,
     which no scalar oracle can model. That contributor's source is now digital silence: it keeps its
     486-sample latency, and its contribution is exact `+0.0`.
   - **Slice 18 gate 4.** It compared live-controlled and control-free fold counts. A fallback to the
     base commit's live-controlled count was added, so a pre-existing difference cannot fail the
     slice.
   - **C ABI offsets.** `maximum_submixes` keeps the size at 208 but moves `reserved` from offset 176
     to 184 (pinned at `crates/capi/src/abi.rs:454` and `crates/capi/tests/c/abi_smoke.c:45`).
     V2's `maximum_vcas` moves it again, to 192. DESIGN P12, slice 09 and V2 now say so.
   - **Browser reasons.** Slice 19 adds a second reason, 13 `unknownRoute`, for an out-of-range live
     route index, so the drift mutation moves to 14 (DESIGN 5.11).
   - **`length == 0` mute.** It is inactive in the block whose drain applied it (DESIGN 5.7,
     corrected).

---

## 5. Old-to-new issue map

| Old | New |
|---|---|
| 00 | 00 (adds the #1053 annotation, the CI-route correction and the `AGENTS.md` dual-mono sentence) |
| 01 | 01 (adds the missed sites) |
| 02 | 02 (grammar and wire) + 03 (lowering) |
| 03 | 04 |
| 04 | 05 (adds sidechains) |
| 05 | 06 |
| 06 | 07 (SDK code) + 08 (skill and handoff) |
| 07 | 09 (caps and C ABI) + 10 (meters) + 11, host-core half (master strip) |
| 08 | 11, browser half (boot word) + 12 (frame and enumeration) |
| 09 | 13 (admission and strip mute) + 14 (SDK) |
| 10 | 15 (mute) + 16 (follow-mute) + 26 (domains, owner-gated) |
| 11 | 17 (render plane) + 18 (host-core producer) |
| 12 | 19 (admission) + 20 (enumeration and SDK); the metadata family moves to 26 |
| 13 | 25 |
| 14 | 22 (native row) + 23 (V8 document) + 24 (baseline and decision) |
| 15 | deferred O1 (filed only on slice 24's go) |
| 16 | deferred O2 |
| 17 | deferred O3 |
| 18 | deferred O4 |
| 19 | merged into 24 (padding) and Q5 (order) |
| 20 | 21 |
| 21 | V1 (schema) + V2 (preparation) |
| 22 | V3 |
| 23 | V4 |
