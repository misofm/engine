# Decision 15 streams

Index of the work filed under decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`), written on
2026-10-05 against `main` at `6fb211594`. The evidence for each decision is in this folder:
`PLAN-2026-10-05-root-draft.md`, `PLAN-2026-10-05-adversary-round1.md`,
`PLAN-2026-10-05-adversary-round2.md` and `PLAN-2026-10-05-agreed.md`. Every issue body is the
local spec in `.github/ISSUE_SPECS/` and equals its GitHub body.

## How the streams run (D15-0)

- Each stream is one opus-high coordinator in its own worktree, with opus-medium workers and fresh
  opus-xhigh verifiers. A stream edits only the files it owns, plus the named exceptions listed for
  it; every exception is also named in the issue that needs it.
- Root merges into `main` one stream batch at a time, rebases the others onto the result, and runs
  the full gate set once on every merged tree.
- At most five implementation coordinators run at once: A, B, G, J and one of E, I or H(a).
- Start immediately (no file conflicts): A's #1300, B's #1309, E's #1055, G's #1328, H's #1331
  and #1333, I's #1335, J, K.
- Size: the fix round split #1225, #1288, #1290, #1296, #1310, #1312, #1327, #1341, #1355, #1358
  and #1381, and round 5 split #1287's first slice (the claim-line carry and fill went to #1402) and #1355
  (the control plane's classification, publication and record went to #1403). Round 6 split #1358:
  the default ring and every pin of the ring rule went to #1406, and the deadline step stayed.
  The verifiers still flagged #1280, #1316, #1320, #1325, #1332, #1333, #1334, #1354, #1355, #1363
  and #1397 as tight; their stream coordinator splits any that does not fit half a day before
  implementation (AGENTS.md).
- **Stream C design gate.** Every adversarial round from round 2 on found new defects in the warm
  successor, and round 5 replaced its design with prime adoption (decision 15, "Verification";
  D15-8 (round-5 amendment)). Before its first implementation slice, stream C's coordinator runs
  one fresh opus-xhigh design verification of #1287, #1320, #1354, #1355, #1358, #1360, #1361,
  #1396, #1397, #1402, #1403 and #1406 together, and folds every BLOCKER and MAJOR into the specs (and
  GitHub) first. Its checklist includes the round-6 folds: the C1 restart walk over every edge kind
  (#1354 D2 step 6), routing a warm growth on #1324 D4's duck set (#1403 D2, #1397 D1), and the
  donor republished on a refused deadline re-preparation (#1358 D3).
- Critical path: S0, then B #1309, then B #1312 (cells) and #1343, then A's carry slices (#1277 onward)
  and B #1398/#1310/#1311, then {C, D, F, H #1332}, then H's SDK slices. Cells precede the carry
  slices that write into them (#1312 before #1277, #1345 before #1280). Inside C: #1287, then
  #1402 and (after D #1324) #1354, then #1355, then #1403, then #1397; A's #1286 measures on #1355, and
  #1406, #1358 and #1360 take its constants, so #1361 comes last.
- Umbrellas (no stream; root keeps them current): #1269 *Swap a rebuilt plan without an audio gap*
  and #1053 *Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes*. Their slice tables mirror the issues below.
- The order inside each stream table is a topological order of the "## Dependencies" sections:
  "After (same stream)" and "After (other streams)" list each issue's dependencies that are open.

## Hot files: merge order

A file edited by more than one stream is merged in this order; a later stream rebases. The order
never overrides an issue's "## Dependencies": where they seem to disagree, the dependencies govern.

| File | Order |
|---|---|
| `crates/host-core/src/prepare.rs` | B #1312 → A (#1277-#1285) → B #1344 → A #1323 → H #1401 → C (#1402, #1396) → D (#1288, #1324, #1325) → C (#1354, #1355, #1397, #1406) → F |
| `crates/effect-compiler/src/prepare.rs` | I #1335 (starts immediately) → B (#1315, #1345) → G (#1339, #1340, #1377, #1378); F #1306 after B #1345, either order with G, the second rebases |
| `crates/builtins-compiler/src/lib.rs` | B (#1312, #1346) → A (#1277) → D (#1288) → G (#1329) → F (#1261, #1262); J #1418 (two region markers around `drain_controls`) and J #1420 (the test module only) in any order, the later slice rebases; J #1423 (the strip record types) after B #1312 and #1346 |
| `crates/effect-contract/src/live.rs` | B #1312 → B #1345 → A #1280 → E #1341 |
| `crates/effect-contract/src/lib.rs` | J #1330 → G #1377 |
| `crates/engine/src/realtime/plan_exchange.rs` | B #1343 → B (#1310, #1311, #1314) → C (#1396, #1355) → H #1381 → B #1349 |
| `crates/engine/src/realtime/plan.rs` | B #1311 → H #1400 → C #1396 → H #1381 |
| `crates/engine/src/realtime/spsc.rs` | B (#1343, #1311) → C #1320 (`peek` only) |
| `crates/source/src/lib.rs` | B (#1318, #1316, #1350, #1319, #1343, #1344) → C (#1320, #1355) |
| `crates/host-core/src/transition.rs` | D (#1325, #1324) → C #1397 |
| `crates/host-core/src/live_delta.rs` | I #1335 (starts immediately) → B (#1312, #1345-#1347) → A (#1277, #1280) → E (#1054, #1394, #1365, #1341) → F → G #1371; J #1423 (strip record construction only) after B #1312 and #1346, rebasing over the rest |
| `crates/control-plane/src/*` (after #1309) | B → H #1400 (`RuntimePreparer`) → A #1323 → D #1325 → C (#1396, #1355, #1403, #1397, #1358, #1360) → F → H #1381 |
| `crates/capi/include/miso_engine_v1.h` | B (#1318, #1314, #1316) → B #1317 → B #1348 → A (#1285, #1323) → D (#1288, #1324, #1325) → C #1360 |
| `crates/graph/src/{lib,runtime}.rs` | A → B (#1344, #1347) → C (#1287, #1402, #1396) → D (#1288, #1363) → C #1355 → G #1371 |
| `crates/graph-compiler/src/*` | A #1285 → J #1384 → C #1287 first slice → G #1379; J #1415 (`ids.rs`'s route constants only) in any order, the later slice rebases |
| `crates/parametric-eq/src/lib.rs` | A payload (#1279, #1280) → G #1328 (rest predicates only) → G #1337 → G #1372 |
| `hosts/host-web/src/lib.rs` | H owns; J #1423 (record construction in `into_track_record` and the coalescing sites) lands after B #1312 and #1346; B (#1312, #1345-#1347, #1399), D #1326 and E #1364 land before H #1332 rewires the worklet; B #1349 lands after H #1381; F #1306 lands when #1058's design allows and never blocks H; C #1406 edits one doc comment (the ring override field's) after H #1381 |
| `sdk/src/core/session.ts` | I #1335 → E #1364 → H #1385 |
| `sdk/src/core/live-controls.ts` | E (#1054, #1364) → G #1369 → H #1382 |
| `tools/parameter-metadata/src/abi_layout.rs` | H (#1333, #1380, #1332, #1381, #1293, #1386) and B (#1399, #1349) → C #1406 (the `sourceRing` rule only) |
| `scripts/check-abi-layout-v1.py` | H (#1333, #1332, #1381, #1293, #1386), B #1349 and G #1378 → C #1406 (the `sourceRing` check only) |
| `hosts/host-web/src/tests.rs` | about 30 slices of streams B, D, E, F, G and H edit it; C #1406 (`default_ring_covers_stall_tolerance` and the doc comment of `ring_prefill_survives_stall` only) lands after H #1381 and rebases over any later edit |
| `hosts/host-web/web/miso-engine-v1-audio-worklet-host.{js,d.ts}`, `sdk/src/browser/shipped-host.d.ts` | H (#1332, #1294) and B (#1399, #1349) → C #1406 (the ring's in-flight bound and comments only, wherever #1332 leaves them) |
| `scripts/check-web-audioworklet-callgraph.py` | J #1234 and H #1333: either order, the second rebases; then J #1417 and H #1333 the same way |
| `scripts/build-web-audioworklet.sh` | H #1334 → H #1380 → H #1332 |
| `rust-toolchain.toml`, `.github/workflows/*.yml` | #877 (stable bump) and H #1334 (nightly entry): either order, #877 never touches the browser-artifact entry; J #1422 (two doctest steps in `qualification.yml`) and J #1429 (the `artifact-gates` "Hermetic browser host and worklet tests" step) in any order, the later slice rebases |

## Stream S0

- **Coordinator scope:** Decisions and specs (root).
- **Owns:** `docs/rulings/`, `AGENTS.md`, `.github/ISSUE_SPECS/`, `docs/handoffs/decision-15-2026-10-05/`.
- **Depends on:** none
- **Parallel-safe with:** all.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1308 | Record decision 15: live updates, seamless swaps and one control plane | — | — |

## Stream A

- **Coordinator scope:** Swap carry on the C ABI core: move-mode carry for every state owner, latency floors and their reset, meter carry, and the swap block's cost record (#1286).
- **Owns:** `crates/builtins*`, `crates/rack`, `crates/graph`, `crates/graph-compiler` (except #1384), effect crates' payload code, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`; by named exception: `crates/host-core/src/live_delta.rs` record functions (#1277, #1280, #1284), `crates/host-core/src/spectrum.rs` (#1327), the `control-plane` crate after #1309 (#1280, #1323). By named exception, for #1323 D3's `SuccessorBase::new` only: the `SuccessorBase` construction sites in `crates/host-core/tests/support/successor.rs`, `crates/host-core/tests/withdrawn_successor.rs` (B's, #1344) and `crates/capi/tests/resource_lifecycle.rs` (B's).
- **Depends on:** S0; B #1312 before #1277 and #1345 before #1280 (cells first); #1323 needs #1309, #1310, #1311 and #1314; #1286 needs C #1354 and #1355.
- **Parallel-safe with:** G, J, K, H(a); B once #1312 and #1345 have landed.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1300 | Let soft-clip restore its own non-finite history | — | — |
| 2 | #1277 | Carry fader, mute and pan ramps across a plan swap | — | #1312 |
| 3 | #1279 | Carry console effect lanes across a plan swap | #1277 | — |
| 4 | #1280 | Carry live-controlled effect lanes across a plan swap | #1279 | #1312, #1345 |
| 5 | #1281 | Carry an insert lane that moves between a bank and a per-node instance | #1280 | — |
| 6 | #1282 | Carry per-node effect instances across a plan swap | #1281 | — |
| 7 | #1283 | Carry compensation lines across a plan swap | #1282 | — |
| 8 | #1284 | Carry strip delay lines and live send ramps across a plan swap | #1283 | — |
| 9 | #1285 | Keep every node's latency from dropping during playback | #1284 | — |
| 10 | #1286 | Record the swap block's cost on the 64-track console | #1284 | #1331, #1354, #1355 |
| 11 | #1327 | Carry meter and effect observation state across a plan swap | #1284 | — |
| 12 | #1323 | Reset latency floors at a host-declared discontinuity | #1285 | #1309, #1310, #1311, #1314 |
| 13 | #1395 | Carry spectrum capture state across a plan swap | #1327 | #1401 |

## Stream B

- **Coordinator scope:** Control plane and protocol: the portable `control-plane` crate, candidate withdrawal and supersession, scheduled adoption, latest-target cells, path field, watermark and service, typed refusals, seek contract, telemetry defects.
- **Owns:** new `crates/control-plane`, `crates/capi` (incl. `include/miso_engine_v1.h`), `crates/protocol`, `crates/engine/src/realtime`, `crates/host-core/src/live_delta.rs`, `crates/source` (seek parts); by named exception: `crates/graph` route lanes (#1347), `crates/host-core/src/prepare.rs` (#1344), `hosts/host-web` cell and status code (#1312, #1345-#1347, #1349), `crates/effect-compiler/src/prepare.rs` (#1315, #1345), policy and audit scripts each spec lists.
- **Depends on:** S0; #1344 needs A #1277; #1349 needs H #1381 and #1399.
- **Parallel-safe with:** A, G, J, K, H(a).

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1309 | Extract the C ABI control plane into a portable crate both hosts call | — | — |
| 2 | #1344 | Prepare a successor across a withdrawn candidate plan | — | #1277 |
| 3 | #1318 | Report held source blocks apart from underruns | — | — |
| 4 | #1305 | Find live C ABI edit targets without linear scans | #1309 | — |
| 5 | #1313 | Report each transaction's edit path in its response | #1309 | — |
| 6 | #1315 | Refuse commands that would be acknowledged with no effect | #1309 | — |
| 7 | #1343 | Let the control thread withdraw an unadopted candidate plan | #1309 | — |
| 8 | #1398 | Size the C ABI's plan capacities and resource admission for a superseding candidate | #1309 | — |
| 9 | #1314 | Publish an applied-revision watermark and complete edits asynchronously | #1309, #1343 | — |
| 10 | #1310 | Supersede an unadopted candidate plan by compare-and-swap | #1309, #1314, #1343, #1344, #1398 | — |
| 11 | #1311 | Adopt a successor plan no earlier than a scheduled sample | #1309, #1314, #1343 | — |
| 12 | #1316 | Anchor every seek on the plan's source-read clock | #1314, #1318 | — |
| 13 | #1348 | Add miso_engine_v1_service for bounded control work between edits | #1309, #1311, #1314 | — |
| 14 | #1317 | Document the seek contract and the C ABI growth rule in the header | #1316, #1318 | — |
| 15 | #1350 | Tighten the seek entry points: source.id.invalid, a typed held preparation, timed reads only | #1316 | — |
| 16 | #1312 | Hold live values in latest-target cells on both hosts | #1309, #1348 | — |
| 17 | #1319 | Test held seeks across swaps and supersession, and add a seek to audit capi | #1310, #1348 | — |
| 18 | #1351 | Report each configured counter's own value in the C ABI counter snapshot | #1309, #1348 | — |
| 19 | #1346 | Hold strip input-lane values in latest-target cells | #1312 | — |
| 20 | #1347 | Hold route-lane values in latest-target cells | #1312 | — |
| 21 | #1399 | Report live_values_superseded in the browser status and prove both hosts drain strip cells alike | #1312 | — |
| 22 | #1352 | Report each configured meter handle's own meter in the C ABI meter batch | #1309, #1351 | — |
| 23 | #1345 | Hold effect parameter, bypass and EQ-target values in latest-target cells | #1312, #1399 | — |
| 24 | #1349 | Publish the applied-revision watermark in the browser status | #1309, #1314, #1348, #1399 | #1381 |

## Stream C

- **Coordinator scope:** Latency growth by prime adoption (D15-8 (round-5 amendment)): lead floors and source-claim lines, their carry and fill, the source-read clock, the readiness check and raw-frame prime, warm preparation, adoption at the first ready block, the duck-swap of restarted strips, the transition fallback and its deadline, and the service wiring on both hosts. There is no off-thread executor, catch-up, peek pool, copy-mode carry or render-thread pre-roll.
- **Owns:** `crates/source` readiness and prime (#1320) and the source driver's two prime methods (#1355), after B's #1316-#1319, #1343, #1344, #1350; `crates/engine/src/realtime/spsc.rs` `peek` only (#1320); `crates/engine/src/realtime` source-read clock (#1396), after #1310, #1311, #1314, #1343; host-core warm-successor code (`crates/host-core/src/warm.rs`, new) and its rows in `prepare.rs`, `source.rs` and `transition.rs` (`grown_strips`, #1397), after streams A and D; the control-plane growth path and service step (#1355 tests, #1396's declaration term, #1403, #1358, #1360, #1397); `crates/capi` and `tools/audit/src/capi.rs` for #1360, with `docs/C_ABI_V1_QUALIFICATION.md`; the source-report assertions in `crates/host-core/tests/` and `crates/capi/tests/`, and `Cargo.lock`, for #1320; `crates/host-core/tests/source_read_clock.rs` (new) for #1396; `hosts/host-web` for #1361. By named exception in stream H's files, for #1406's ring rule only: `hosts/host-web/src/tests.rs` (`default_ring_covers_stall_tolerance` and the doc comment of `ring_prefill_survives_stall`), the ring override field's doc comment in `hosts/host-web/src/lib.rs`, `tools/parameter-metadata/src/abi_layout.rs` and `tools/parameter-metadata/tests/abi_layout.rs`, `scripts/check-abi-layout-v1.py` and `scripts/fixtures/abi-layout-v1-self-test.json`, `sdk/src/core/abi.ts` (`defaultSourceRingFrames`), the regenerated `sdk/assets/miso-engine-v1-abi-layout.json` and `sdk/src/generated/abi.ts`, the ring assertions of `sdk/test/{boot,browser,builder,render}-evals.mjs`, the per-source in-flight bound in `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` and its quantum-64 case in `scripts/test-web-audioworklet.mjs`, the ring comments of `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` and `sdk/src/browser/shipped-host.d.ts`, rows 7 and 8 of `hosts/host-web/MUTATIONS.md`, the ring sentence of `hosts/host-web/DEPLOYMENT.md`, the ring comment of `scripts/web-mixing-automation-benchmark.mjs`, the ring label in `docs/derivations/241-browser-source-identities.md`, the qualification record's ring label in `hosts/host-web/qualification/{qualification.js,run.mjs,generate-matrix.mjs}` with the key rename in `results.json` and the regenerated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, and a note in `docs/derivations/243-sdk-boot.md`; `crates/capi/src/runtime/tests.rs` (B's) for the same assertions. #1287's first slice edits `crates/graph` and `crates/graph-compiler` after #1285; #1402 edits `crates/graph` and `prepare.rs` after #1283.
- **Depends on:** A (#1277, #1283, #1285, #1286, #1323, #1327, #1395), B (#1309, #1310, #1311, #1313, #1314, #1316, #1318, #1319, #1343, #1344, #1348, #1349, #1351, #1398), D (#1288, #1324, #1325), and for #1361 H (#1290, #1293, #1294, #1331, #1332, #1333, #1381).
- **Parallel-safe with:** E, F, G, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1287 | Grow latency during playback by adopting a primed warm successor | — | #1285 |
| 2 | #1402 | Carry source-claim lines across a plan swap and fill a grown line for a prime | #1287 | #1283, #1285 |
| 3 | #1396 | Give a plan a source-read clock that leads its render clock | — | #1316, #1323 |
| 4 | #1320 | Let a source consumer check and replay its next blocks for a prime | — | #1316, #1318, #1319 |
| 5 | #1354 | Prepare a warm successor whose carried nodes lead the predecessor by P | #1287, #1396, #1402 | #1277, #1285, #1323, #1324 |
| 6 | #1355 | Adopt a warm successor with a raw-frame prime at the first ready block | #1287, #1320, #1354, #1396, #1402 | #1277, #1310, #1311, #1314, #1323, #1327, #1343, #1344, #1395 |
| 7 | #1406 | Grow the default source ring by the warm-prime headroom | #1354, #1355 | #1286 |
| 8 | #1403 | Classify a latency-growth edit and publish its warm successor from the control plane | #1320, #1354, #1355, #1396 | #1310, #1311, #1313, #1314, #1323, #1324, #1325, #1343, #1348, #1398 |
| 9 | #1397 | Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm successor cannot adopt | #1354, #1355, #1396, #1403 | #1288, #1310, #1311, #1314, #1324, #1325, #1343, #1344, #1398 |
| 10 | #1358 | Fall back to the transition when a warm successor is not ready by its deadline | #1354, #1355, #1396, #1397, #1403, #1406 | #1310, #1314, #1323, #1325, #1343, #1398 |
| 11 | #1360 | Check the warm-successor deadline in miso_engine_v1_service and report its outcome | #1354, #1355, #1358, #1397, #1403, #1406 | #1286, #1309, #1311, #1313, #1314, #1323, #1348, #1351, #1398 |
| 12 | #1361 | Check the warm-successor deadline in the browser Worker's service loop and report its outcome | #1355, #1360, #1403, #1406 | #1290, #1293, #1294, #1331, #1332, #1333, #1349, #1381 |

## Stream D

- **Coordinator scope:** Transitions (D15-9): fade-in, two-phase removal, duck-swap, route ramps, live strip lanes on every browser plan.
- **Owns:** builtins fade flag, `crates/graph` fire table (#1288, #1363), host-core successor code, host-web lane request (#1326); `control-plane` structural arm (#1325) after B.
- **Depends on:** A #1277, #1283-#1285; B #1309-#1314; E #1054 (through #1288/#1325 specs).
- **Parallel-safe with:** F, G, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1288 | Fade in a strip that a swap adds during playback | — | #1054 |
| 2 | #1326 | Give every browser plan live strip fader and mute lanes | — | — |
| 3 | #1363 | Ramp a route that a plan swap adds to or removes from a surviving strip | #1288 | #1054, #1283, #1284, #1285, #1310, #1314 |
| 4 | #1391 | Give every route whose tap precedes its strip's fader a live lane on every plan | #1326 | #1225, #1347 |
| 5 | #1392 | Keep an added strip's pending fade-in across a later plan swap | #1288, #1363 | #1277, #1283, #1284 |
| 6 | #1325 | Remove a strip in two phases: ramp out, then a scheduled swap | #1288, #1363, #1391 | #1054, #1309, #1310, #1311, #1312, #1313, #1314, #1347 |
| 7 | #1324 | Duck-swap a strip whose state cannot continue across a plan swap | #1288, #1325, #1363, #1391 | #1277, #1310 |

## Stream E

- **Coordinator scope:** Smoothness: researched ramp defaults, every live value ramps, bypass crossfade.
- **Owns:** `crates/session`, `LiveRamps` in `crates/host-core/src/live_delta.rs` (after B), `sdk/src/core/live-controls.ts`, `crates/effect-contract/src/live.rs` (after #1280 and #1312); `crates/protocol` opcode (#1365) after B.
- **Depends on:** S0; #1341 needs #1280 and #1345; #1364 needs #1335.
- **Parallel-safe with:** A, B, G, J, K.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1055 | Research: default ramp lengths for live mute, fader and pan changes (cited, measured, listened) | — | — |
| 2 | #1054 | Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes | #1055 | — |
| 3 | #1394 | Carry an optional per-edit ramp length on live session edits | #1054 | — |
| 4 | #1364 | Resolve an absent live ramp to the session default on the browser and in the SDK | #1054 | — |
| 5 | #1365 | Edit control_smoothing by a session transaction, model-only | #1054 | — |
| 6 | #1341 | Crossfade the bypass switch over the session ramp | #1054, #1055 | #1280, #1345 |
| 7 | #1388 | Run the blinded listening session for the live ramp defaults | #1054, #1055, #1364 | — |
| 8 | #1393 | Crossfade the browser's live bypass command over the session ramp | #1341, #1364 | — |

## Stream F

- **Coordinator scope:** C ABI live completeness: sends, follow-mute, VCAs, input section, submix strips, span windows.
- **Owns:** host-core classifier (`crates/host-core/src/live_delta.rs`) and lane attachment, `crates/control-plane/src/{control,compile}.rs` after #1309, `crates/capi` tests; #1306 deletes `hosts/host-web/src/lib.rs:6574-6591` (H's file).
- **Depends on:** B #1309, #1312, #1346, #1347; G #1328, #1329.
- **Parallel-safe with:** C, D, H.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1225 | Deliver value-only send edits to the running C ABI plan | — | #1054, #1277, #1284, #1309, #1312, #1313, #1347, #1394 |
| 2 | #1261 | Apply value-only input trim and polarity edits to the running C ABI plan | — | #1054, #1309, #1312, #1328, #1329, #1346, #1394 |
| 3 | #1268 | Elide a builtin input filter section again after a live disable settles it to identity | — | — |
| 4 | #1306 | Size each effect's automation span window from the producers its plan has | — | #1058, #1304, #1309, #1345 |
| 5 | #1390 | Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan | #1225 | #1054, #1277, #1309, #1312, #1394 |
| 6 | #1262 | Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets | #1261, #1268 | #1312, #1328, #1329, #1346 |
| 7 | #1226 | Let C ABI sends follow their source strip's mute live | #1225, #1390 | #1054, #1309, #1312, #1347, #1394 |
| 8 | #1267 | Apply value-only submix-strip input-section and effect edits to the running C ABI plan | #1261, #1262, #1390 | #1309, #1312, #1345, #1346 |
| 9 | #1247 | Deliver value-only VCA edits to the running C ABI plan | #1225, #1226, #1390 | #1054, #1309, #1312, #1347, #1394 |

## Stream G

- **Coordinator scope:** DSP contracts: SVF joint flush, engine-wide tail and exact-rest bounds, live gate/EQ/multiband parameters, live bypass shunts, per-strip and multiband link mode.
- **Owns:** `crates/lane`, `crates/dsp-reference`, effect crates' parameter and designer code, `crates/effect-runtime` (#1366, #1375); by named exception: `crates/parametric-eq` rest predicates (#1328), `crates/builtins-compiler` tail rule (#1329), `crates/graph-compiler` extent (#1379), `crates/host-core/tests/live_delta.rs` rows (#1336, #1337, #1367), `sdk/` (#1369), classifier rows (#1371).
- **Depends on:** A's carry slice for each effect crate (#1279, #1280, #1282); #1069 for the multiband; E #1054 for #1371.
- **Parallel-safe with:** A (coordinate on effect crates), B, E, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1328 | Flush the SVF jointly so builtin and EQ filters reach exact rest | — | — |
| 2 | #1336 | Make the gate-expander's attack, hold and release live | — | #1279, #1280 |
| 3 | #1337 | Make a parametric EQ band's enabled and kind live | — | #1279, #1280 |
| 4 | #1366 | Prove the crossover designer total and share the SVF ramp stability check in effect-runtime | — | — |
| 5 | #1339 | Give the delay a live bypass shunt | — | #1282, #1315, #1341 |
| 6 | #1368 | Lower the link mode to per-lane state in the linked effects' banks | — | #1279, #1280 |
| 7 | #1329 | State a bounded tail and an exact-rest bound for every node | #1328 | — |
| 8 | #1338 | Make the multiband compressor's crossover live | #1366 | #1069, #1280, #1282 |
| 9 | #1340 | Give the multiband compressor a live bypass shunt | #1339 | #1069, #1280, #1282, #1315, #1341 |
| 10 | #1369 | Declare a strip's console link mode in the session, the wire and the SDK | #1368 | — |
| 11 | #1370 | Ramp a lane's detector link between modes | #1368 | — |
| 12 | #1377 | Carry each effect's tail and exact-rest bound in its prepared metadata | #1329 | — |
| 13 | #1371 | Carry the link record from the edit to the lane | #1369, #1370 | #1054, #1279, #1280, #1312, #1345, #1364, #1394 |
| 14 | #1379 | Define how node tails compose through gain in the graph extent | #1329, #1377 | #1237 |
| 15 | #1367 | Make the multiband compressor's link mode live | #1371 | #1069, #1280, #1282 |
| 16 | #1372 | State the parametric EQ's bounded tail and exact-rest bound | #1328, #1329, #1377, #1379 | — |
| 17 | #1375 | Report a zero tail beyond latency for the compressor and the true-peak limiter | #1377, #1379 | — |
| 18 | #1236 | Let a strip override a console slot's link mode | #1367, #1368, #1369, #1370, #1371 | #1054, #1196, #1279, #1280, #1345, #1394 |
| 19 | #1373 | State the multiband compressor's bounded tail and exact-rest bound | #1329, #1338, #1375, #1377, #1379 | — |
| 20 | #1374 | State the delay's bounded tail and exact-rest bound | #1375, #1377, #1379 | — |
| 21 | #1376 | State exact-rest bounds for the gate, transient shaper and soft clip | #1375, #1377, #1379 | — |
| 22 | #1378 | Retire the Infinite tail | #1372, #1373, #1374, #1375, #1376, #1379 | — |

## Stream H

- **Coordinator scope:** Browser control plane: shared-memory spike, allocation gates, nightly browser artifact, Worker control plane, render-only worklet, transaction API in the SDK.
- **Owns:** `hosts/host-web`, `sdk/`, `scripts/check-web-audioworklet*`; #1334 also `rust-toolchain.toml`, `.github/workflows/{qualification,npm-publish}.yml`, `docs/RELEASE.md`, `docs/TARGET_MATRIX.md` and the build/identity scripts it lists; #1333 also `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py`; #1381 the `RuntimePreparer` hook in `crates/control-plane`.
- **Depends on:** B #1309, #1312-#1316, #1348, #1349; A #1277, #1327; D #1326; E #1054, #1364; F #1225, #1226, #1247, #1261, #1262, #1267; I #1335.
- **Parallel-safe with:** A, G, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1401 | Prepare every browser preparation branch concurrently, as a successor too, in host-core | — | #1326 |
| 2 | #1333 | Gate AudioWorklet render against allocation statically and at runtime | — | — |
| 3 | #1383 | Build and encode session transactions in the SDK | — | #1394 |
| 4 | #1331 | Prove two Wasm instances on one shared memory in three browser engines and on iOS | — | — |
| 5 | #1400 | Prepare through an adapter-supplied preparer in the control-plane crate | #1401 | #1309, #1326 |
| 6 | #1385 | Encode the session, submix, output, route, automation and VCA edits in the SDK | #1383 | #1335, #1394 |
| 7 | #1334 | Build the browser artifact on a pinned nightly toolchain | #1331 | — |
| 8 | #1380 | Ship the browser module with one imported shared memory at every instantiation site | #1331, #1333, #1334 | — |
| 9 | #1332 | Run the browser control plane in a Worker and keep the AudioWorklet render-only | #1331, #1333, #1334, #1380 | #1057 |
| 10 | #1387 | Move browser source submission and seeks into the Worker | #1332 | #1316, #1318 |
| 11 | #1381 | Swap and retire browser plans through the Worker's service loop | #1332, #1387, #1400 | #1309, #1314, #1327, #1348, #1395 |
| 12 | #1382 | Admit browser live edits in the Worker through the committed model | #1332, #1381 | #1054, #1057, #1225, #1226, #1247, #1261, #1262, #1267, #1312, #1313, #1345, #1346, #1347, #1364, #1390, #1394 |
| 13 | #1290 | Replace the running browser session in the Rust host | #1332, #1381, #1382, #1387, #1400, #1401 | #1277, #1309, #1310, #1313, #1314, #1326, #1327, #1348, #1349, #1395 |
| 14 | #1293 | Export transaction apply and anchored seek from the browser engine module | #1290, #1332, #1381, #1387 | #1309, #1313, #1316, #1319 |
| 15 | #1342 | Make a send's follows_mute live in the browser | #1290, #1382 | #1226, #1313, #1347 |
| 16 | #1386 | Diff a replacement document against the committed model and export replace from the browser engine module | #1290, #1293 | — |
| 17 | #1294 | Send a session transaction to the browser control plane | #1293, #1332, #1381, #1382, #1386, #1387 | #1310, #1348, #1349 |
| 18 | #1295 | Qualify a structural browser edit in real browsers | #1290, #1294, #1332, #1333, #1386 | — |
| 19 | #1296 | Apply session transactions from the browser SDK | #1294, #1295, #1382, #1383, #1385, #1386 | #1312, #1313, #1314, #1325, #1326, #1349 |
| 20 | #1297 | Feed and retire the sources a browser edit adds or removes | #1293, #1296, #1332, #1381, #1387 | #1316, #1325 |
| 21 | #1389 | Apply session transactions from the headless SDK engine | #1293, #1296, #1332, #1381, #1382, #1383, #1385, #1386 | — |

## Stream I

- **Coordinator scope:** Effect automation guard (D15-13 E1).
- **Owns:** effect-compiler validation, `sdk/src/core/session.ts`.
- **Depends on:** S0
- **Parallel-safe with:** all except H's SDK slices (#1385 after #1335).

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1335 | Refuse automation on effect parameters that are not block-rate | — | — |

## Stream J

- **Coordinator scope:** Hygiene and preparation performance.
- **Owns:** the scripts, tests and files each issue lists; `crates/effect-contract` (#1330); `crates/graph-compiler` and `crates/graph/src/program.rs` (#1384, after #1285).
- **Depends on:** S0; #1303 and #1384 need A (#1277, #1285).
- **Parallel-safe with:** all.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1248 | Name the failed predicate and bound the waits of the browser continuous-spectrum gate by a deadline | — | — |
| 2 | #1251 | Make concurrent tests fail instead of hanging when a thread panics | — | — |
| 3 | #1384 | Resolve graph node IDs to dense indices once per graph compilation | — | #1285 |
| 4 | #1232 | Make the C ABI checker's header mutation legs reach the compiler | — | — |
| 5 | #1330 | Validate each effect descriptor once per type, not once per prepared instance | — | — |
| 6 | #1235 | Remove the dead code builtins-compiler reports under no-features clippy | — | — |
| 7 | #1234 | Anchor the worklet callgraph checker's C allocator names | — | — |
| 8 | #1301 | Make the shared edge-ramp restore probe cheap enough for every pull request | — | — |
| 9 | #1302 | Make the realtime-policy drain rule structural instead of one regex line | — | — |
| 10 | #1303 | Route the builtins fader domain checks through checked_fader_gain | — | #1277, #1312 |
| 11 | #1304 | Tighten the four-lane reference graph ceilings from measured AArch64 rows | — | — |
| 12 | #1237 | Bound route gain and matrix values | — | — |
| 13 | #1415 | Spell the route gain and matrix domain once, in the session model | — | — |
| 14 | #1417 | Refuse a C allocator name anywhere in an unmangled worklet symbol | — | — |
| 15 | #1418 | Require every loop around a realtime drain to drain a different queue on each pass | — | — |
| 16 | #1419 | Use the shared StopOnDrop in the C ABI plan-swap race test | — | — |
| 17 | #1420 | Make the nonadjacent split-pair harness's track choices observable to its tests | — | — |
| 18 | #1421 | Remove both temporary directories the web AudioWorklet test script creates | — | — |
| 19 | #1422 | Run doctests in CI | — | — |
| 20 | #1416 | Let only host-core build the live route records that hosts push | #1422 | — |
| 21 | #1423 | Make live strip records valid by construction | #1422 | #1312, #1346 |
| 22 | #1426 | Bound each realtime drain per queue: require the pop receiver to be the counted queue | #1418 | — |
| 23 | #1429 | Fail the web AudioWorklet test step when it leaves anything in its temporary directory | #1421 | — |
| 24 | #1434 | Hold every block-form owner step in qualification.yml to its guard lines | #1429 | — |

## Stream K

- **Coordinator scope:** Research and design notes.
- **Owns:** `docs/handoffs/`.
- **Depends on:** none
- **Parallel-safe with:** all.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1057 | Design: one edit API on every host over the core's committed session model | — | — |
| 2 | #1058 | Research: render stored session automation in the engine, identically on every platform | — | — |
