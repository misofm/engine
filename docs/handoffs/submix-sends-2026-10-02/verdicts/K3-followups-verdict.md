# K3 follow-ups verdict (Sol): `843e27558` (parent `eff44271d`), branch `codex/batch-submix-k3`

**FAIL for the push. The follow-up commit itself passes review.**

- **The follow-up commit is correct.** Every ledger item is applied or justifiably skipped. The
  #1224 MINOR-1 patch is correct and adds no click or leak. The doubled route-ID charge is right.
  The new `KERNEL_ROSTER` row bites. `route_values` moves no bit. The rustdoc fix works.
- **K3 is not ready to push.** One BLOCKER makes CI's required `cross-target` job fail. It came
  from #1220 attempt 1 (`d405abb37`), not from the follow-up commit, and no per-slice verifier ran
  `check-cross-targets.sh`. A bit-identical three-line fix is proved below. Once it lands and is
  recorded, K3 is ready to push.
- **Five MINORs and some NITs.** None of them blocks the push. MINOR-1 (a test gap in the patch)
  should ride with the BLOCKER fix. MINOR-2 to MINOR-5 are spec and ruling text edits; each one
  needs its GitHub body re-synced.
- **Worktree.** I left it clean at `843e27558` and restored every mutation. Probes and mutations
  ran in `git archive` copies. The only things I did in the worktree were builds and a `touch` to
  force fresh clippy and rustdoc runs. Nothing was committed or pushed, and nothing was edited on
  GitHub.

## BLOCKER-1: `check-cross-targets.sh` fails on HEAD (`ios-asm-memset-pattern16`, graph 11 > 10)

- **Symptom.** `bash scripts/check-cross-targets.sh` exits 1:
  `graph: memset_pattern16 calls rose from 10 to 11 (#1018)`, then
  `cross-target check failure: ios-asm-memset-pattern16 moved`. The ceiling is
  `IOS_MEMSET_CEILINGS["graph"] = ("1018", 10)` in `scripts/lib/aarch64-known-defects.py`.
- **Impact.**
  - The route is `full`, so the verdict job expects `cross-target` = success. The K3 PR and the
    `main` push would both fail `qualification`.
  - The script stops at the first `fail`, so its later rows did not run on HEAD: the iOS and
    Android eight-lane scans and the wasm refusal and simd rows. They pass on HEAD plus the fix
    (below).
- **Bisect.** I counted the graph crate's `aarch64-apple-ios` release assembly, as the script emits
  it, for each K3 commit:

  | Commit | Count |
  |---|---|
  | `cfa086d4a` (main) | 10 |
  | `071c6c14a`, `1b5034c35`, `0268a1c74` | 10 |
  | `d405abb37` (#1220 attempt 1) | **11** |
  | `466f0ab63`, `eff44271d`, `843e27558` | 11 |

- **Cause.**
  - The new call sits in `lane::kernels::route_mix_ramp_block::<f32x4>`, instantiated in `graph`:
    `adrp x1, l_.memset_pattern.2653` ... `bl _memset_pattern16`.
  - Its pattern is `0x40800000` x 4, that is `4.0`: `let advance = L::splat(L::WIDTH as f32)`,
    which LLVM stores to a stack slot through the libc call.
  - This is the #1018 class: a libc call inside a render kernel on iOS. The ratchet exists to stop
    new ones.
- **Fix (proved).** Derive each chunk's frame-index vector from a `u32` counter, so there is no
  splatted constant to store. Every index is an exact integer below `2^24`, so the values, and
  every bit, are unchanged:

  ```diff
  -    let advance = L::splat(L::WIDTH as f32);
  -    // Exact: `position < length <= 2^22` whenever a frame ramps.
  -    let mut index = L::splat(position as f32).add(L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]));
  +    let offsets = L::load(&FRAME_INDEX_OFFSETS[..L::WIDTH]);
  +    // Exact: `position + vectored < length <= 2^22` whenever a frame ramps.
  +    let mut first = position;
       for (left, right) in left_vectors
           .chunks_exact_mut(L::WIDTH)
           .zip(right_vectors.chunks_exact_mut(L::WIDTH))
       {
  +        let index = L::splat(first as f32).add(offsets);
  +        first += L::WIDTH as u32;
           ...
  -        index = index.add(advance);
       }
  ```

  With exactly this edit on a copy of HEAD:
  - the graph iOS count is back to **10**;
  - `check-cross-targets.sh` passes in full (rc 0). `cross-target matrix: PASS`, with graph at 10
    and every other memset row at its ceiling. The iOS and Android eight-lane scans and the wasm
    rows pass too;
  - `build-web-audioworklet.sh --named-twin` and `check-web-audioworklet.sh` pass. The roster row
    reads `route-mix-ramp f32x4 vector=21 scalar=0 budget=8.0`, the render closure is still
    `closure=8 traps=5` with sole owner `render_inner`, and there are 13 kernels;
  - these tests are green: `lane --test route_ramp` (4), `graph-compiler --test live_routes
    --test route_coefficients`, all of `graph` (`test-support`), and host-web's send, follow and
    route tests (19).
- **Land it as a K3 follow-up, recorded as an amendment to #1220.** In the same edit:
  - update the comment numbers that say 22 to 21: the `KERNEL_ROSTER` table row and the
    `SCALAR_SLACK` comment in `check-web-audioworklet-callgraph.py`, and #1220's NIT-1 record;
  - add `check-cross-targets.sh` to the per-slice LESSON, next to `check-web-audioworklet.sh`, for
    any slice that touches `lane`, `graph` or an effect crate.
- **Raising the ceiling to 11 is not an equivalent fix.** It would admit a libc call into a render
  kernel on iOS, and it needs #1018's owner to agree.

## MINOR findings (none blocks the push)

### MINOR-1: the amended D3 rule's `Both` arm is untested

- **Mutation.** `BuiltinLaneSelector::Both => changed[0] && changed[1]` (it should be `||`) passes
  every committed host-web test: the lib suite's 164, `boot_transient_budget` and
  `retained_ceilings`.
- **Effect.** A both-lane mute of a strip that already has one lane muted and a following send is
  refused (`RESULT_INVALID_ARGUMENT` from the `malformed` guard). That is a user-visible
  regression, and nothing catches it.
- **Why.** P3 covers only single-lane records.
- **Probe.** My V4 turns the mutation red. In it, bass starts `[F,T]`; one batch carries
  `mute bass Both @400`, a no-op `mute bass R @0` and `mute drums R @64`; it is compared bit for
  bit with the same batch without the no-op.
  - V4 is also red on `eff44271d`'s rule, and green at HEAD.
  - Adopt it, or a single-record version of it, as a committed test.
- The source is `K3-followups-verifier-scratch.rs`, next to this file.

### MINOR-2: #1236 has no deliverable that amends `AGENTS.md` and decision 12

- `AGENTS.md` is binding, and it says every strip carries every console slot "with only its own
  parameters and bypass".
- Decision 12's ruling declares `link_mode` once, on the slot (`engine-footprint-2026-09-29.md:50`).
- DESIGN 8.2 Q1 itself lists "a schema change to decision 12's slot declaration" as a cost.
- L2 (D2) changes exactly that. Give L2 (or the umbrella) a deliverable that amends the
  `AGENTS.md` console sentence and records the change against decisions 12 and 13. Use decision
  13's qualifier convention while it is unlanded. Then re-sync the GitHub body.

### MINOR-3: the decision-13 ruling's Q1-Q4 bullets name no authority kind

- **The convention.** The ruling's header says "Each bullet names whose authority it carries",
  using four kinds. The new bullets do not.
- **Q1** is *owner direction, read by the planner*. "Follow whatever modern day DAWs look like" is
  the owner's direction; "a strip may override its console slot's `link_mode`" is the planner's
  reading of it. That kind also selects the "Planned under decision 13 ..., subject to owner
  review" qualifier for `AGENTS.md`.
- **Q4** is a planner default that the owner did not object to (the owner asked for an
  explanation). It is not an owner decision.
- **Q2 and Q3** read as owner decisions. Check whether the subnormal flush in Q2 was itself put to
  the owner. The ledger says it was.
- **NIT, Q1.** "so a per-strip override never splits a bank" is #1236 L1's goal, not today's
  fact. `link_mode` is in `EffectProgramKey` today, as #1236's own "Today" section says. Say
  "would not need to split".
- **Not verifiable by me.** The root ledger quotes only Q1, and its quote matches the ruling's.
  The Q2-Q4 quotes ("This makes sense to add bounded min/max for these values right?", "There's no
  way around this right?", "Please explain this to me.") are presented as verbatim. The root
  should confirm them against the owner thread.
- **Not overstated.** Q4's "raised no objection" and Q5's "open, pending" are accurate.

### MINOR-4: #1234 has a stale anchor and a gate that cannot pass as written

- **The anchor.** The spec says "every anchor here is verified on the batch K3 follow-up tree".
  But `RouteControlProducer::free` is not at `crates/host-core/src/route_controls.rs:59-68` on
  HEAD. It is at `:71-80`, with the attribute at `:77`. The follow-up's 12-line `# Lifetime`
  rustdoc moved it. `:59-68` was right at `eff44271d`.
- **The gate.** Gate 4 runs `bash scripts/check-web-audioworklet.sh <A>` with one argument. The
  script requires `ARTIFACT_DIRECTORY NAMED_TWIN` and exits 2 with a usage message
  (`scripts/check-web-audioworklet.sh:156-159`). Write `<A> <N>/miso-engine-v1-audio-worklet.simd128.named.wasm`.
- Fix both in the spec and re-sync the GitHub body.

### MINOR-5: #1237's gates miss the worklet and iOS gates for the crates it edits

- #1237 edits `graph` (`gated_route_coefficients`) and `graph-compiler`, which compile into the
  shipped worklet and the iOS product crates. Its gates omit:
  - `build-web-audioworklet.sh --named-twin` with `check-web-audioworklet.sh` and
    `test-web-audioworklet.sh`;
  - `check-cross-targets.sh`.
- The K3 LESSON and BLOCKER-1 are exactly this class. Add them.
- **NIT, #1236.** When its slices are filed, they should name the same gates, plus the
  `KERNEL_ROSTER` rows of the effects they touch and `run-aarch64-tests.sh` (CI).

## NITs (optional)

1. `ISSUE-MAP.md`: the new line "filed V4 must keep it. Owner question Q2 was answered ..." is
   about 118 columns.
2. #1235 cites `BoundaryVariant` at `:5876-5878`. That range includes `NonadjacentTrackA` (`:5877`),
   which is live. The dead variants are at `:5876` and `:5878`.
3. #1237 cites `crates/host-core/src/route_controls.rs:82` for the producer's check. That line is
   `free()`; `record` is at `:86`.
4. #1236 "Today" cites `effect-contract/src/lib.rs:1171` "onward" for the struct, which is at
   `:1173`. Fine as written.

## Ledger items

| Item | Status |
|---|---|
| Retire 1206-1214 | Done. They are CLOSED (PR #1233, `cfa086d4`, upstream). Their specs are gone from `.github/ISSUE_SPECS/`. `ISSUE-MAP.md` links all nine at `cfa086d4a8f8...`, and every path exists there. Only the archived verdict copies still name the old paths, which is correct for a record. |
| 1215 MINOR-1 (open fold wording) | Done in #1215 D1, #1216 D2, DESIGN 5.7 and the ruling's #1215 bullet. |
| 1215 MINOR-2 | Done. The #1216 verdict ran "the runtime ignores the gate" red under gates 1 and 4 (verdict lines 26 and 168-174). |
| 1215 NITs | Done. `1.0e35` is correct (10^(700/20)). `route_values` derives the transform once (see below). `const fn` is kept. The INFO went to Q2/#1237. |
| 1216 NITs, INFO | Done (wording, test-value sentences, INFO for the silence work). |
| 1217 MINOR-1 (must land before push) | Landed in #1220 as host-core `a_middle_inactive_route_keeps_every_active_contribution` (row 1220-11, MC red). NIT-1 and NIT-2 are done. |
| 1218 MINOR-1 | Done. **I reproduced it:** swapping `mute`/`follows_mute` in `parse_route` turns `every_route_and_automation_opcode_round_trips_canonically` red. |
| 1218 MINOR-2/3, NIT-1/2/3 | Done: schema and spec sentence, inherited row, `expect`, the #1058 note, and the follow-only estimate case. |
| 1219 MINOR-A | Done. The `kernels.rs` doc is the verdict's replacement text, word for word. `BUILTINS_AND_METERING_V1.md:275` carries the same bound. |
| 1220 MINOR-1 | Done and correct (see below). |
| 1220 MINOR-2/3 | Done. #1235 is filed. **I reproduced its claim** on HEAD: the no-features clippy fails with exactly the two `dead_code` errors. |
| 1220 NIT-1 (roster row) | Done, and it bites (see below). NIT-2/3 have no change, as the verdict allowed. |
| 1221 MINOR-2, NIT-3, #1225 additions | Done. The gate-6 numbers are corrected, the lifetime rustdoc is on `RouteControlProducer`, and #1225 has the acked-at-a-swap gate and the cross-route skew statement. |
| 1221 rustdoc (must fix before push) | Done. **I confirmed** `cargo doc -p graph-compiler` with `-D warnings` fails at `eff44271d` (`route_coefficients` links the private `route_transform`, `ids.rs:318`). The workspace doc is green at HEAD (forced fresh, 38 crates). |
| 1222 MINOR-1..5, ratification, #1234 | Done. **I spot-checked V6** (mirror commit dropped): red in gate 2 and #1224's full-queue test, as recorded. |
| 1223 MINOR-1/2 | Done. The NITs are not applied, with reasons. |
| 1224 MINOR-1..4 | Done, and correct (see below). There is one test gap (MINOR-1 above). NIT-1..3 are not applied (optional). |
| Owner answers Q1-Q4 | Recorded, and #1236 and #1237 are filed. See MINOR-2, MINOR-3 and MINOR-5. Q5 is open in both #1196 and the ruling. |

## #1224 MINOR-1 patch: correct, no new click or leak

- **Reasoning.** At batch start, the route mirror's `source_lane_muted` equals the solo state's
  emitted lanes. After every admitted batch, both equal the effective mute.
  - So a lane whose effective mute changed was moved by a staged `Fader::Mute` record on that
    strip's slot that covers it: either a kind 4 record or a coalescing record.
  - So the `malformed` guard stays unreachable.
  - `[..follow_start]` drops only follow records, which are never strip mutes, so it has no
    semantic effect.
  - "Last covering record wins" matches the strip: `set_mute` retargets on every record, so a later
    record's window is the one the lane obeys.
- **Probes on HEAD, all green:**
  - P1-P4, plus the #1224 verifier's A, B, C (randomized, 300 batches) and D;
  - my V1: two strips, interleaved lane mutes with no-ops, and a solo in the same batch, compared
    block for block with explicit `routeMute` records at the expected windows (bass-room 0,
    drums-verb 200); then an un-solo batch, settled and bit-identical to a booted host;
  - my V1b (un-solo plus a no-op on a settled lane), V3 (a no-op re-mute of an already-muted lane,
    with a solo in the batch) and V4 (a `Both` record, then a no-op single-lane record).
- **On `eff44271d`, V3 and V4 are red** (`0.274 vs 0.561` and `1.082 vs 1.145` at sample 0). V1
  and V1b are green on both commits, which is expected: their last record already covers a
  changed lane.

## #1220 doubled route-ID charge: correct

- **What it charges.** Attach holds two heap copies of each route ID: the producer's, and the
  `GraphRouteControlBinding`'s node ID. Bind frees the bindings. So `2 x len` is exact for the
  attached state and an upper bound after bind. That matches `owner_bytes`, which already charges
  the binding struct after bind.
- **The rustdoc** ("an upper bound on both"; every field exact except `activity_bytes`) is accurate.
- **The new test bites.** With a single count, the attach retains 1,804 B against a charge of
  1,694 B, and both `route_control_resources_cover_the_attached_state` and gate 8 go red. With the
  double count the charge is 1,948 B, so the slack is 144 B. That is neither an under-charge nor
  a material over-charge.

## `KERNEL_ROSTER` row: bites

- **Mutation.** I made `route_mix_settled_tail` `#[inline(always)]` and rebuilt the module.
  - With the row, the kernel-shape rule fails:
    `FAIL roster route-mix-ramp f32x4: vector=22 scalar=18 budget=8.0`.
  - Without the row, the same module passes (rc 0).
- **At HEAD** the row reads `vector=22 scalar=0`.

## `route_values` refactor: no bit moved

- **By construction.** The old lowering ran `route_coefficients(..).ok()` (which checks
  `route_transform`, then the open fold) and then `route_transform`. The new one runs
  `route_values`: the same checks, returning the same transform.
- **Compile dump.** One probe, the same at `eff44271d` and HEAD, built 800 drawn sessions from the
  nine-track fixture plus sends. It drew strip lane mutes, `mute`, `follows_mute`, gains from
  -150 to 40 dB plus 700/-144/24/0, and coefficients including +/-0, `1e10`, `3e38` and `1.2e-38`.
  - It dumped every canonical graph text digest, every prepared route's gate and gated
    coefficient bits, and every refusal's diagnostics.
  - Both commits produced byte-identical dumps: 476 sessions compiled, 324 refused with identical
    diagnostics, and 8,085 prepared routes in the second run.
- **Render digests.** Seventeen scenarios were identical across the two commits:
  - the #1222 send session with live gain, matrix and mute records;
  - the #1224 solo-follow session under all 16 session lane-mute combinations of two strips,
    with live lane mutes, a solo and an un-solo.
- **Checked-in evidence.** `graph_fixture --check`, graph determinism (100/100), the builtins and
  console fixtures, and the browser expected-resources PCM digests are all green on HEAD.

## Specs and GitHub

- **Open issues.** `gh issue list --state open` (110) equals `.github/ISSUE_SPECS/` (110). No
  issue is missing either way.
- **Bodies and titles.** The bodies of #1196, #1215-#1226 and #1234-#1237 are byte-equal to their
  local files, and each title equals its H1. All are OPEN. #1206-#1214 are CLOSED.
- **Placement.**
  - #1234, #1235 and #1237 are standalone. That is sensible: tooling, hygiene and one bounded
    product slice.
  - #1236 is a small umbrella with two named slices, L1 and L2, filed later, outside #1196. That
    is sensible too: #1196's closure is fixed, and L1 is a cross-effect kernel change worth
    splitting.
- **Owner answers.**
  - #1236 reflects Q1 ("follow whatever modern day DAWs look like"): per-instance link choice
    becomes a per-strip override, for tracks and buses alike.
  - #1237 reflects Q2: `[-144, 24]` dB, `[-1, 1]`, subnormal flush to `+0.0`, with the metadata
    half deferred.
- **Anchors.** #1236's anchors check out on HEAD: the model, parse, lowering, `EffectProgramKey`,
  banks, compressor kernel, eligible list, link-mode sets, the #1087 precedent and the schema's
  "no optional fields" line. #1237's check out apart from NIT 3. I also re-counted its migration
  claim: 312 routes in checked-in JSON, none out of the new domain. #1235's check out. #1234 has
  one stale anchor (MINOR-4).
- **Verdict copies.** The 21 K3 verdict and probe copies in
  `docs/handoffs/submix-sends-2026-10-02/verdicts/` are byte-equal to `/home/bl/misofm/submix-verdicts/`.

## Local gate set on HEAD `843e27558` (x86-64-v3, rustc 1.97.1, Node 22.23.2)

The router gives `route=full`, `math_closure=true`, `release_inputs=false` and
`self_tests=["sdk-deletions"]`. Every step was rc 0 except `cross-target` (BLOCKER-1).

- **`route`:** `check-ci-path-routing.py` and `test-ci-path-routing.py`.
- **`docs-gates`:** the DSP research and listening gates.
- **`artifact`:** `build-web-audioworklet.sh --named-twin`.
  - Module `4b0c2fb3...1e15` (2,765,142 B); named twin `8410ed88...` (3,155,514 B). Both match
    the #1224 record.
  - **Reproducible:** an `archive` copy with a fresh `CARGO_HOME` built the same bytes.
  - `web-audioworklet-identity.py report`: `ARTIFACT CHANGED f08c5433... -> 4b0c2fb3...`,
    Reproducible, rc 0. Its self-test passes.
- **`sdk`:**
  - generated, deletions and types;
  - headless: 355/355;
  - package check.
- **`artifact-gates`:**
  - strip-names self-test and check;
  - `check-web-audioworklet.sh --without-metadata-regeneration`: render `closure=8 traps=5`, 13
    kernels, all 11 roster rows ok;
  - `check-browser-expected-resources.py --artifacts` (32 red mutations);
  - wasm scalar-absent;
  - `test-web-audioworklet.sh`;
  - V8 spill self-test and gate.
- **`browser`, CI mode** (SDK source bundle, `sdk/dist` deleted first, private PulseAudio null
  sink, `--check-matrix --self-test-mutations`): chromium 151.0.7922.34, firefox 153.0 and webkit
  26.5. All qualification gates passed.
- **`lint`:**
  - fmt;
  - clippy `--all-features -D warnings`, rerun fresh after a `touch`, 38 crates;
  - rustdoc `-D warnings`, fresh, 38 crates;
  - every policy check and mutation pair: workspace, session, env vocabulary, bench, test-support
    CI, script reachability, host-core, protocol-control, realtime plus audit-leak plus
    artifact-evidence-leak, trace validator, lane, unfused seal, rack/builtins/graph and
    effect-runtime with its fixtures;
  - scalar-oracle self-test, console benchmark fixture, builtins fixture mutations and bench
    preconditions;
  - conformance boundaries, PEQ seal, release-shape self-test, npm publish modes, stem store and
    stem identity;
  - the three x86 lane-guard probes.
- **`test-debug-a`:** the exact CI command: 108 binaries, **1,239 passed**, 0 failed, 9 ignored.
- **`test-debug-b`:** 146 binaries, 791 passed, 0 failed, 24 ignored, plus
  `conformance_fixtures --check`.
- **`test-release`:**
  - lane, math and wasm-gates: 29 binaries, 105 passed;
  - the M3 FMA cfg;
  - loom `spsc_loom`;
  - **M1 exhaustive and F1 exhaustive**, which run because `math_closure=true`.
- **`audit-native`:**
  - the release build;
  - release audit, bench and console-workload tests: 110 passed;
  - `audit capi` plus the CI validator: 0 allocations and 0 syscalls, digest `ff6cdcb96cdcdad5`;
  - the delay, compressor, PEQ and gate audits;
  - the builtins, builtins-graph and graph traces (1M blocks), and the protocol allocation audit;
  - the realtime probes, the 1M trace and the builtins probes;
  - the effect-contract trace;
  - `check-capi-abi.sh` and `--self-test`;
  - `libcapi.so` scalar-absent;
  - graph determinism 100/100 and `graph_fixture --check`;
  - builtins fixtures (50) and console fixtures;
  - effect contract.
- **`wasm-guests`:**
  - the simd128 probe and the evidence crates;
  - `check-protocol-wasm-parity.sh`;
  - the wasm-gates build and `run-wasm-gates.sh --without-v8-spill --without-native`.
- **`cross-target`:** **FAIL** (BLOCKER-1). With the fix applied, it passes in full.
- **`gate-self-tests`:** `check-sdk-deletions.py --self-test`.

**Not run:**
- `aarch64-debug` and `aarch64-release`: there is no arm64 host, so they run in CI at the push.
- The release unwind check: `release_inputs=false`.
- nightly and fuzz.

## What blocks the push, and the order to unblock

1. Apply BLOCKER-1's kernel edit to `crates/lane/src/kernels.rs` and the 22 -> 21 comment numbers.
   Record it as a #1220 amendment, or as a K3 follow-up. Adopt MINOR-1's V4 test in the same
   commit.
2. Re-run `check-cross-targets.sh`, `build-web-audioworklet.sh --named-twin` with
   `check-web-audioworklet.sh`, and the route tests. The module digest will move: that is an honest
   ARTIFACT CHANGED.
3. Fix the MINOR-2 to MINOR-5 texts, and re-sync the #1234, #1236 and #1237 bodies (and the ruling)
   on GitHub.
4. K3 is then ready to push, as a PR merged into `main`.
