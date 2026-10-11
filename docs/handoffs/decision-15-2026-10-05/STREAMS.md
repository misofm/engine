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
- Start immediately (no file conflicts): A's #1300, B's #1309, E's #1055, G's #1328, #1407, #1408
  and #1409 (after #1408; J's #1301 has landed), H's #1331, #1333, #1449 and #1448, I's #1335, J,
  K. #1449 and #1448 each change functions in a hot file and come first in its row below. B's #1447
  starts when stream B batch 1 is on `main`.
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
- Critical path: S0, then B #1309, then B batch 1 (#1343, #1314, #1311, #1348), then B batch 2
  (#1432 → #1502 → #1503 → #1504 → #1312: the revision-bounded cells, root
  2026-10-05), then A's carry slices (#1277 onward) and B #1398/#1310, then {C, D, F, H #1332},
  then H's SDK slices. Cells precede the carry slices that write into them (#1312 before #1277,
  #1345 before #1280). Inside C: #1287, then
  #1402 and (after D #1324) #1354, then #1355, then #1403, then #1397; A's #1286 measures on #1355, and
  #1406, #1358 and #1360 take its constants, so #1361 comes last.
- Umbrellas (no stream; root keeps them current): #1269 *Swap a rebuilt plan without an audio gap*
  and #1053 *Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes*. Their slice tables mirror the issues below.
- The order inside each stream table is a topological order of the "## Dependencies" sections:
  "After (same stream)" and "After (other streams)" list each issue's dependencies that are open.
- **The realtime-policy floors and allowlists (standing exception for every stream).** Any slice
  in any stream may edit, in `tools/realtime-policy`'s `Policy::workspace()`, only these: the
  region and file floors, when it adds or removes a marker; and the control-side allowlist, when it
  adds, moves or removes a non-test `try_pop` outside a marked region (one row per function: path,
  canonical function, site count, thread and reason; a render-thread function is never listed). It
  names the edit in its spec. Every other edit of `tools/realtime-policy` is stream J's. Each slice
  re-measures the floors when it lands, whatever landed before it.
  - **Until #1446 is on `main`,** the same slice may instead (or also) edit the awk gate's two floor
    lines (`scripts/check-realtime-policy.sh:75-76` on `b8392df66`, `:73-74` on `6d28a80ec`) and,
    because the awk self-test's base tree sits exactly on the floors, the self-test's pad loop, its
    comment and the matching floor messages (`scripts/test-realtime-policy.sh:445-454`, `:1408`,
    `:1410`, `:1414` on `b8392df66`), as `98a2d6bfc` authorized for #1314. Without that edit the
    self-test exits 1 ("expected at least ninety-three marked realtime regions").
  - **Not covered:** the unsafe allowlist (`Policy::workspace()`, and `:29` of the awk gate until
    #1446). A new file with `unsafe` code needs its own named exception, as today.

## Hot files: merge order

A file edited by more than one stream is merged in this order; a later stream rebases. The order
never overrides an issue's "## Dependencies": where they seem to disagree, the dependencies govern.

No other slice builds on `store_revision`, `active_revision`, `RevisionTarget` or `set_revision`
(#1502 deletes them), so a slice in another stream must not call them.

| File | Order |
|---|---|
| `crates/host-core/src/prepare.rs` | B #1312 → A (#1277-#1285) → B #1344 → A #1323 → H #1401 → C (#1402, #1396) → D (#1288, #1324, #1325) → C (#1354, #1355, #1397, #1406) → F; G #1379 (the tail copy and the gain-liveness selection passed to the graph compile, a few lines) in either order with the others, the later slice rebases; G #1469 (the `launch_native_effect_registry` call only, one line) in either order with the others, the later slice rebases; G #1457 (the internal policy function's `bound_cache` parameter, its `None` arguments and one unit test) in either order with the others, the later slice rebases; G #1471 (the two C ABI entry points' `bound_cache` parameter) after #1457 |
| `crates/lane/src/kernels/builtins.rs` (the D11 trim, fader and matrix ramp kernels), the ramp stages in `crates/builtins/src/lib.rs` (`InputStage::set_trim_signed`/`settle`, `FaderRampStage`, `MatrixStage`), `crates/dsp-reference/src/ramp.rs` | G #1408 → G #1409 → B (#1312, #1346) → E (#1054, #1394) → A #1277 → D #1288 |
| `crates/builtins/src/lib.rs` (`InputStage::apply_prepared_filter`), `docs/rulings/builtins-input-liveness-d2.md` (#808 paragraph) | G #1407 → G #1329 → F (#1268, #1262) |
| `crates/lane/src/kernels.rs` (`ramp_block`, the `ramp_toward` re-export), `crates/effect-runtime/src/ramp.rs`, `crates/effect-runtime/src/state_payload.rs` (`ramp_path_inside`), the effect ramp render bodies (compressor `RampVec`/`advance_ramps`, gate `channel_step`, multiband `Segment`/`run_segment`, delay `LaneChunk`/`CrossChunk`/`delay_chunk`, soft clip `SoftClipCoef`/`soft_clip_block`/`process`, limiter `RampLanes`) | G #1409 → A (#1279, #1280, #1282) → G (#1336, #1338, #1370); G #1455 (multiband `run_segment` and its per-segment dispatch) after #1409, in either order with #1338 (neither depends on the other), the later slice rebases |
| `crates/effect-runtime/src/state_payload.rs` (`ramp_path_inside`'s walk, `ramp_path_within`), the effect payload readers' ramp range rules (compressor `state.rs` `validate_channel`, gate `parse_lane`, multiband, delay `read_carried_ramp`, transient shaper and limiter `read_lane`, limiter `coefficient_bounds`, soft clip `ramp_current_valid`), their restore-refusal test rows (delay `a_carried_ramp_is_refused_unless_its_whole_path_is_valid`, limiter restore corruptions, `crates/compressor/tests/payload.rs`), soft clip's overshoot restore tests (`tests/state_roundtrip.rs`, `tests/randomized.rs`), `crates/delay/tests/MUTATIONS.md` (M18, M19), the #1301 spec | J #1301 → G #1409 → G #1411 (#1409 and #1411 in one Stream G pull request); A's carry slices (#1279, #1280, #1282) and that pull request in either order, the second rebases |
| `crates/effect-compiler/src/prepare.rs` | I #1335 (starts immediately) → G #1377 → G #1461 → G #1462 → B (#1315, #1345) → G (#1339, #1340, #1378); F #1306 after B #1345, either order with G, the second rebases. #1377 goes first (root, 2026-10-06, #1377 Amendment 1): B and G's other slices are blocked, and #1377's edit is one comparison and one test; the later slices rebase. #1461 and #1462 follow #1377 by the same reason and land before #1372 (root, 2026-10-06). G #1464 (the `composition` comparison and its forgery row) after G #1462, and G #1484 (the forgery row's `stall` literal, the field rename only) after G #1464, in either order with B (#1315, #1345); the later slice rebases. G #1469 (the process-lifetime launch registry: its static, builder and entry point, `launch_registry_owns_factory`, and the factory-charging loop of `effect_control_resources`, which now skips a registry-owned factory) after G #1462 and before G #1372, in either order with G #1464 and B (#1315, #1345); the later slice rebases |
| `crates/builtins-compiler/src/lib.rs` | B (#1504 signatures only, #1312, #1346) → A (#1277) → D (#1288) → G (#1329) → F (#1261, #1262); J #1441 (two markers around `impl BuiltinBankProcessor`, replacing blank lines; if a blank line is gone, J #1443 places them) and J #1443 (markers around the three test-support `LiveControl*` drains, and the move of `LiveControlInputProcessor`'s drain into its own `drain_controls`), in any order with A, B (#1312, #1346), D, G and F, the later slice rebases; J #1420 (the test module only) in any order, the later slice rebases; J #1423 (the strip record types) after B #1312 and #1346; G #1464 (the bound type, the seal, `input_bounds`, the tail-entry charge and their unit tests) after G (#1329), in either order with F (#1261, #1262) and J, the later slice rebases |
| `crates/effect-contract/src/live.rs` | B #1312 → B #1345 → A #1280 → E #1341 |
| `crates/effect-contract/src/lib.rs` | J #1330 → G #1377 → G #1461 → G #1462 (root, 2026-10-06; both before #1372) → G #1464 (the `EffectTailBound` → `NodeTailBound` rename, the `composition` field, the two registry rules) → G #1484 (the stall split into `peak_stall` and `tail_stall`, the two clause readers, registry rule (g), `max`'s stall rule; after G #1465, before G #1466); G #1469 (the feature-gated `tail_and_rest` evaluation counter in `NativeEffectRegistry::new` only) after G #1462, in either order with G #1464, the later slice rebases; G #1409 (`ParameterSmoother`'s `Linear` arm only) in either order with J #1330, the later slice rebases |
| `crates/engine/src/realtime/plan_exchange.rs` | B #1343 → B (#1314, #1311) → B (#1502, #1503) → B #1310 → C (#1396, #1355) → H #1381 → B #1349; B #1482 (the `note_claim` call only) after #1314, in any order with the rest, the later slice rebases |
| `crates/engine/src/realtime/plan.rs` | B #1311 → B #1502 → H #1400 → C #1396 → H #1381 |
| `crates/engine/src/realtime/spsc.rs` | B (#1343, #1314, #1311) → B #1502 (the revision words go) → C #1320 (`peek` only) |
| `crates/source/src/lib.rs` | B batch 1 (#1343) → B #1447 → B (#1318, #1316, #1350, #1319, #1344) → C (#1320, #1355); J #1443 (three region markers, each replacing a blank line, around `take_recycled_block`, `observe_seek_at_block_boundary` and `acquire_current_block` with #1447's `settle_block`) after B #1447, in any order with the rest; the later slice rebases |
| `crates/host-core/src/spectrum.rs` | H #1449 (the `SpectrumCapture` drains and the collection's `cancel_except`, by named exception) → A #1327 → A #1395; J #1443 (markers around the spectrum drains and `cancel_except`, replacing blank lines where there are some) after H #1449, in any order with A #1327 and #1395; the later slice rebases; A #1479 (`entry` and `selected_entry`, the private `owned_entry` and `select`'s two `entry` call sites (body only, root's Amendment), outside every marked region) in any order with the others, the later slice rebases; H #1492 (`select`'s and `selection_would_change`'s signatures, `owned_entry`'s removal and the new `SpectrumTargetRef`, outside every marked region) after A #1479, in any order with the rest, the later slice rebases |
| `crates/host-core/src/transition.rs` | D (#1325, #1324) → C #1397 |
| `crates/host-core/src/live_delta.rs` | I #1335 (starts immediately) → B (#1312, #1345-#1347) → A (#1277, #1280) → E (#1054, #1394, #1365, #1341) → F → G #1371; J #1423 (strip record construction only) after B #1312 and #1346, rebasing over the rest; G #1469 (the registry plumbing and `load_registry` only) in any order with the others, the later slice rebases |
| `crates/control-plane/src/*` (after #1309) | B (batch 1, then #1502, #1503, #1312) → H #1400 (`RuntimePreparer`) → A #1323 → D #1325 → C (#1396, #1355, #1403, #1397, #1358, #1360) → F → H #1381; `control.rs` is also edited by G #1371 and E #1394 (084754c4b re-anchored their paths from capi), each after B #1312 |
| `crates/capi/include/miso_engine_v1.h` | B (#1318, #1314, #1316) → B #1317 → B #1348 → A (#1285, #1323) → D (#1288, #1324, #1325) → C #1360 |
| `crates/graph/src/{lib,runtime}.rs` | G #1460 → G #1461 (the bank record's `metadata` field and `stage_for`'s read; test effects) → G #1462 (one test request's `tail_bound` field) → A and B #1504 (the drain signatures): either order, the second rebases; #1504 lands before A's #1277, which needs it through #1312 → B (#1344, #1347) → C (#1287, #1402, #1396) → D (#1288, #1363) → C #1355 → G #1371; G #1464 (the rename only, `lib.rs`) and G #1379 (`GraphNode`'s field and the test literals that build one, `lib.rs`), each after G #1461, in either order with A, B, C and D, the later slice rebases |
| `crates/rack/src/lib.rs` | A's slices before #1280 and B #1504 (the drain signatures): either order, the second rebases → B #1345 → A #1280 |
| `crates/graph-compiler/src/*`, `crates/graph-compiler/tests/scale.rs` | G #1460 (`estimate.rs` live-control owner charge and its `lib.rs` test mirror) → G #1461 (`banks.rs` bind check, `estimate.rs` bank reads, test factories) → B #1504 (`tests/scale.rs` only, if the trait change reaches it) → B #1312 (one test and its doc text) → A #1285 → J #1384 → C #1287 first slice → G #1379; J #1415 (`ids.rs`'s route constants only) in any order, the later slice rebases; G #1469 (test-module `launch_native_effect_registry` call sites only, and only if its return-type change forces them) in any order, the later slice rebases |
| `crates/protocol/src/model.rs` | B #1503 (the revision ceiling) → later B and E slices rebase |
| `crates/parametric-eq/src/lib.rs` | G #1328 (rest predicates, their test-module rows, and the per-frame output-limit check restructured so the dual depth-1 tail does not spill, #1328 Amendment 1 A2; landed before Stream A started) → G #1462 (three test requests' `tail_bound` field) → A payload (#1279, #1280) → G #1337 → G #1372; G #1464 (the `tail_and_rest` rename and field only) before G #1372, in either order with A's payload slices and G #1337, the later slice rebases |
| The other `PreparedEffectMetadata {` literals in the effect crates' payload code and test helpers (`crates/transient-shaper/src/corpus.rs:190` and its peers) | G #1464 (the `composition` field only) in either order with A's payload slices; the later slice rebases |
| `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs`, `crates/math/src/tail.rs` | G #1464 (`builtins` only) → G #1465 → G #1484 (`builtins` only) → G #1466 → G #1467 → G #1468; G #1485 after #1467, in either order with #1468, the later slice rebases; G #1487 after #1485 and #1379; G #1457 before #1464; G #1474 (`math` and `builtins` tail, after #1457) before #1465, in either order with #1464, the later slice rebases |
| `hosts/host-web/src/lib.rs` | H owns; J #1423 (record construction in `into_track_record` and the coalescing sites) lands after B #1312 and #1346; B (#1312, #1345-#1347, #1399), D #1326 and E #1364 land before H #1332 rewires the worklet; B #1349 lands after H #1381; F #1306 lands when #1058's design allows and never blocks H; C #1406 edits one doc comment (the ring override field's) after H #1381; H #1448 (`poll_meters`, unmarked) lands in any order with the B, D, E and J slices that land before H #1332, the later slice rebases; then "H #1448's guard, landed by J after C2" (two markers replacing blank lines, in the J tool batch) after H #1448, in any order with the rest, the later slice rebases |
| `sdk/src/core/session.ts` | I #1335 → E #1364 → H #1385 |
| `sdk/src/core/live-controls.ts` | E (#1054, #1364) → G #1369 → H #1382 |
| `tools/parameter-metadata/src/abi_layout.rs` | H (#1333, #1380, #1332, #1381, #1293, #1386) and B (#1399, #1349) → C #1406 (the `sourceRing` rule only) |
| `scripts/check-abi-layout-v1.py` | H (#1333, #1332, #1381, #1293, #1386), B #1349 and G #1378 → C #1406 (the `sourceRing` check only) |
| `hosts/host-web/src/tests.rs` | about 30 slices of streams B, D, E, F, G and H edit it; C #1406 (`default_ring_covers_stall_tolerance` and the doc comment of `ring_prefill_survives_stall` only) lands after H #1381 and rebases over any later edit |
| `hosts/host-web/web/miso-engine-v1-audio-worklet-host.{js,d.ts}`, `sdk/src/browser/shipped-host.d.ts` | H (#1332, #1294) and B (#1399, #1349) → C #1406 (the ring's in-flight bound and comments only, wherever #1332 leaves them) |
| `hosts/host-web/qualification/sdk-response-entry.ts`, `hosts/host-web/qualification/run.mjs` | H #1476 (the SDK instances' allocation reads and the `sdk-render-allocations` gate) and J #1480 (the continuous-spectrum probe and its predicates) in any order; the later slice rebases |
| `hosts/host-web/MUTATIONS.md` | H #1476, H #1477, A #1479 (row `:563` only), J #1480, H #1491 and H #1492, each adding or editing its own rows, in any order; the later slice rebases. H #1492 also edited A #1479's row `:563` (its count, 4 to 6), ratified by root 2026-10-09 |
| `scripts/test-web-audioworklet.mjs`, `sdk/test/browser-pcm-evals.mjs` | stream H's slices (#1491: `testQualificationBoot` only) and J #1496 (`testProcessor`'s and the feed harness's recording checks only) in any order; the later slice rebases |
| `scripts/check-web-audioworklet-callgraph.py` | J #1234 and H #1333: either order, the second rebases; then J #1417 and H #1333 the same way |
| `scripts/build-web-audioworklet.sh` | H #1334 → H #1380 → H #1332 |
| `rust-toolchain.toml`, `.github/workflows/*.yml` | #877 (stable bump) and H #1334 (nightly entry): either order, #877 never touches the browser-artifact entry; J #1422 (two doctest steps in `qualification.yml`), J #1429 (the `artifact-gates` "Hermetic browser host and worklet tests" step) and J #1435 (the `sdk` "Qualify the SDK package against the shared artifact" step), J #1438 (`audit-native`: one build flag and one step) and J #1446 (`lint`: two lines removed and the step renamed), G #1428 (`test-release` steps in `qualification.yml`; under its amended D2 (R1) the release sweeps move to their own parallel required job, never `nightly.yml`) and G #1379 (one `test-release` step for host-core's `tail_composition`) in any order, the later slice rebases; J #1481 (`nightly.yml`: the `native-vectorization-report` step's `continue-on-error` line and its comment, and the file's header bullet for that job) in any order with the rest, the later slice rebases; B #1499 (the `qualification.yml` steps that run `scripts/build-capi-release.sh` and its LTO check) in any order with the rest, the later slice rebases |
| `Cargo.toml`, `Cargo.lock` | J #1438 (one member and its own lock entry) and B #1447 (the `source` package's `bench-support` dev-dependency line) in any order with C #1320; the later slice rebases |
| `scripts/check-ci-path-routing.py`, `scripts/test-ci-path-routing.py` | J #1446 (one `DEDUPLICATED_OWNERS` entry and its case) in any order with any later edit of those tables, B #1499's conditional owner entry among them; the later slice rebases |
| Realtime-policy floors and allowlists (the awk gate's floor lines in `scripts/check-realtime-policy.sh` and its self-test's pad until J #1446, then `Policy::workspace()` in `tools/realtime-policy`) | B batch 1 (#1314) → J #1438; then, in any order, J's tool slices (#1441: one region; the #1448 guard: one; #1443: ten), B #1345 (Amendment 1: one region or more), B's other cell slices (#1312, #1346, #1347), #1321 (`:55`) and any slice under the standing exception. H #1448 adds none. B #1482 (the shared seqlock's render-side region, if its D4 adds one) lands in any order with these. Each slice re-measures the floors when it lands. While the awk gate is on `main` (until the J batch push), a slice that adds a region raises the awk floor and the self-test's pad with it. `Policy::workspace()` reaches `main` only with the J batch push, which re-measures it. G #1494 (one unsafe-allowlist row per new unsafe test file, by named exception, in whichever gate is on `main`) and J #1496 (a new render-thread export table and rule in `tools/realtime-policy`, after #1446) in any order with these; the later slice rebases |
| #1446's comment lines: `crates/engine/src/realtime/observe.rs`, `crates/engine/src/realtime/watermark.rs` (added by B batch 1), `crates/lane/src/fpenv.rs`, `crates/lane/src/softfma.rs`, `crates/capi/tests/resource_lifecycle.rs`, `crates/host-core/src/lib.rs`, `tools/bench-support/src/lib.rs`, `scripts/check-bench-policy.sh`, `docs/REALTIME_DEPENDENCY_POLICY.md`, `docs/REALTIME_MEMORY.md` | J #1446 (comment lines only, each keeping its line count) in any order with every other slice that edits these files; the later slice rebases. J #1489 (`docs/REALTIME_DEPENDENCY_POLICY.md`, "Unsafe-code ownership", and `crates/lane/src/softfma.rs` comment and `SAFETY` lines; it superseded #1478) and B #1482 (`observe.rs` and `watermark.rs` code, after #1314) in any order with #1446, the later slice rebases and keeps #1446's wording. G #1494 (`softfma.rs` and `fpenv.rs` writer signatures, `SAFETY` lines and doctests; the policy's "Unsafe-code ownership" entries; one `scripts/check-bench-policy.sh` path) and G #1495 (`fpenv.rs` module doc lines; citations only elsewhere) in any order with #1446 and each other, the later slice rebases and keeps #1446's wording. G #1498 (`crates/host-core/src/lib.rs:78`, comment only) and B #1499 (`fpenv.rs` "Realtime properties" comment lines, after #1495; the policy's `fpenv.rs` citations if lines move) in any order with #1446, the later slice rebases and keeps #1446's wording |
| `crates/capi/src/runtime/tests.rs` (B's) | G #1494 (the write call sites and the `allow` attribute) and G #1498 (the doc comment `:3080-3081`) in any order; the later slice rebases |
| `scripts/check-cross-targets.sh` | G #1472 (closed) → B #1499 (the iOS and Android emission commands only; the two lines `scripts/check-ci-path-routing.py:870-872` pins stay byte for byte) |
| `.github/ISSUE_SPECS/*.md` | J #1446's D5 substitutions apply to the specs on `main` when it lands; a spec that lands later gets them at the root's issue-boundary audit |

Hot-file note (2026-10-05, after #1329 attempt 1): #1408 changes the D11 ramp law that the trim,
fader and matrix share, in the ramp kernels and their twin. Stream B's cells (#1312, #1346) and
stream E's ramp lengths (#1054, #1394) feed that law, and A's #1277 and D's #1288 carry or drive
its words, so #1408 lands before any of them changes the ramp code; they rebase onto it. #1407
changes only the live filter retarget in `crates/builtins` (stream A's column, a G exception).
#1409 (filed the same day) gives every effect ramp the same clamp right after #1408, so it also
lands before B, E, A and D change the ramp code, and before the effect slices (A #1279, #1280,
#1282; G #1336, #1338, #1370) that add or carry effect ramps.

Hot-file note (2026-10-05, root decision): #1409 owns #1301's consequences. Stream J's #1301 lands
first; #1409 then deletes the `ramp_path_inside` walk and the restore-refusal rows and soft clip
overshoot tests it supersedes, re-measures #1301's mutation counts (measured on the unclamped law)
and records them in an amendment note on the #1301 spec. #1411 then removes the 64-ulp restore
slack from every effect's payload reader, which #1409's clamp leaves unused.

Hot-file note (2026-10-05, root decision): #1411 moved from stream A to stream G so that #1409 and
#1411 merge together, in one Stream G pull request (#1409's commits first). #1409 alone leaves
#1301's six probe tests unable to catch anything, and main must never carry that window; #1411's
gate 3 makes each probe red again on a render-side mutant. Stream G edits the effect payload
readers for #1411 by named exception.

Hot-file note (2026-10-06, root decision): #1451 edits stream A's `crates/builtins/src/lib.rs`
by named exception at `:1071` (`InputChainCoef::constants`, the `InputChainConstants::new()`
field, removed by its D3), and #1452 edits it at `InputStage::load_filter_leading` (`:1336-1350`)
and its two call sites. As G #1407 and #1408 land before A in the other builtins rows, the order
is G #1451 → G #1452 → A (#1277, #1327 and any later stream A slice that edits that file), which
rebase onto them.

Hot-file note (2026-10-06, root ruling): G #1460 (*Keep only render-read effect fields in the
render node table*) edits stream A's `crates/graph/src/{lib,runtime}.rs` (the per-node effect's
render node, `LiveControlEffect`'s effect field, the response row's prepared bypass) and
`crates/graph-compiler/src/estimate.rs` (the live-control owner charge, with its test mirror in
`crates/graph-compiler/src/lib.rs`) by named exception. It lands before any stream A graph slice
and before G #1377, which needs it (#1377 Amendment 2); A rebases onto it.

Hot-file note (2026-10-06, root ruling): G #1461 (*Keep each effect processor's render memory free
of its prepared metadata*) edits the eight effect crates' `src/lib.rs` (each prepared processor's
metadata copy and `metadata()`), `crates/effect-contract/src/lib.rs` and
`crates/effect-compiler/src/prepare.rs` by named exception. It lands after G #1377 and after any
#1409-family slice already landed (#1409, #1411; #1458 if landed first), and before G #1372-#1376,
which rebase onto it. As implemented (attempt 1, Amendment 1 A1-A2) it also reaches every caller
of `NativeEffectFactory::prepare` and `bind_homogeneous_bank` (the prepare result now carries the
metadata): `crates/rack/src/lib.rs` (the two bank stage constructors), `crates/conformance`,
the effect crates' tests, `crates/host-core`, `crates/capi/tests/resource_lifecycle.rs` (one
budget row), `tools/{audit,bench,console-workload}` and `docs/EFFECT_CONTRACT_V1.md`.

Hot-file note (2026-10-07, root ruling, #1469 Amendment 1): G #1469 edits stream H's
`hosts/host-web/src/tests.rs` by named exception, in one test only
(`effect_control_browser_table_and_payload_reach_exact_budget_gate`: its expected factory term
follows the charge rule), and `crates/host-core/src/lib.rs` (one re-export,
`launch_registry_owns_factory`). It also adds a registry warm-up to
`crates/capi/tests/resource_lifecycle.rs` (`warm_process_lifetime_statics`) and
`crates/graph-compiler/tests/live_routes.rs` (`retained`). Its stop 3 also edits
`crates/host-core/src/control_preparation.rs` (`factory_allocation_bytes` charges no
registry-owned factory, and one unit test) and `crates/host-core/tests/prepare.rs` (one expected
value, `effect_control_report_uses_actual_native_capacity_strings_and_owners`). These edits come
after G #1462, in either order with the other slices that touch these files; the later slice
rebases. A slice that rebases over `effect_control_resources` (G #1464, B #1315, #1345) inherits
the changed charge rule: a plan is charged only the factories it retains.

Hot-file note (2026-10-07, root rulings, #1457 Amendment 2): G #1457 adds `bound_cache:
Option<&mut InputBoundCache>` to `builtins_compiler::prepare_session_builtins_with_live_controls`
and edits, by named exception, only to add `None` at its callers: the tests in
`crates/graph-compiler/src/lib.rs` and `crates/builtins-compiler/tests/allocation_tracker.rs`
(`hosts/host-web/src/tests.rs` and `crates/host-core/tests/prepare.rs` name the function only in
comments and need no edit). It also edits `crates/host-core/src/prepare.rs` (the internal policy
function's parameter and a unit test), `crates/builtins/src/lib.rs` (`input_section_bounds` and
its budgeted form), `crates/builtins/src/tail.rs`, `crates/builtins-compiler/src/lib.rs`,
`crates/math/src/tail.rs` (`fixed_cascade_within`) and `crates/builtins/tests/tail_contract.rs`.
Any slice in flight on those callers rebases and passes `None`.

Hot-file note (2026-10-06, root rulings, #1379 Amendment 1): #1379 is split. G #1464 (*Carry
every node's tail bound in one node-neutral struct*) renames `EffectTailBound` to `NodeTailBound`
and adds the `composition` field, so it edits `crates/effect-contract/src/lib.rs`,
`crates/effect-compiler/src/prepare.rs` (one comparison), `crates/builtins-compiler/src/lib.rs`
(the bound type, the seal, `input_bounds`, the tail-entry charge), `crates/graph/src/lib.rs` (the
rename only), the effect crates' `tail_and_rest` functions and their `PreparedEffectMetadata {`
literals (stream A's payload code: the field only), and `crates/builtins` (the bound type and
entry points) by named exception, after G #1457, #1461 and #1462. G #1465, #1466, #1467 and #1468
edit `crates/builtins/src/tail.rs` and `crates/builtins/tests/tail_contract.rs` (stream A's) in
that order. G #1379 (slice C) then edits `crates/graph/src/lib.rs` (`GraphNode`'s field),
`crates/graph-compiler` at its turn, `crates/host-core/src/prepare.rs` (the tail copy and the
gain-liveness selection), every `GraphBuiltinsCompileRequest {` literal (the new field only, in
`crates/graph-compiler/tests`, `crates/host-core/tests` and `tools/audit/src/fixture_builtins.rs`),
`crates/capi/src/runtime/compile.rs` (one read) and `crates/capi/src/abi.rs` (two doc comments)
(stream B's), the new `crates/capi/tests/tail_every_peak.rs` (stream F's column) and, for D15-4(b)
and (c), the decision-15 ruling's two sentences (stream S0's). Slice C is about one working day,
an explicit exception to the half-day rule (root ruling m7, recorded in #1379). #1375 and #1376
need only #1464's carrier and the contract, not the composed extent (#1379 Amendment 1, third
round), so they follow #1465; they land on `main` in the batch with #1379, never before it
(confirmed by root, 2026-10-06), because
before #1379 the extent does not compose a newly finite effect tail through gain. The full
amendment is kept in `1379-amendment1-design.md` in this folder.

Hot-file note (2026-10-08, root ruling on #1466 attempt 1, option (2)): G #1484 (*Split a node's
flush stall into a peak stall and a tail stall*) is a contract change between G #1465 and G #1466.
It edits `crates/effect-contract/src/lib.rs` (`CompositionBound`, its readers, `NodeTailBound::max`,
`tail_bound_consistent`), the forgery row's literal in `crates/effect-compiler/src/prepare.rs`
(the field rename only), `crates/builtins/src/tail.rs` (`stated_composition`, `memoryless_channel`)
and `crates/builtins/tests/tail_contract.rs` (stream A's, named exceptions), the tail-entry
resource pins only where a byte count moves, and the H1 text of #1379's spec and the design note.
It blocks #1466, #1467 and #1379. G #1485 (*Tighten the live input section's peak gain toward the
real-kernel history peak*) is the live `G_p` successor: it edits `crates/math/src/tail.rs`,
`crates/builtins/src/tail.rs` and `tail_contract.rs` after #1467, in either order with #1468, and
re-pins the graph digests its tighter value moves only if #1379 has landed first.

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
- **Owns:** `crates/builtins*`, `crates/rack`, `crates/graph`, `crates/graph-compiler` (except #1384), effect crates' payload code, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`; by named exception: `crates/host-core/src/live_delta.rs` record functions (#1277, #1280, #1284), `crates/host-core/src/spectrum.rs` (#1327), the `control-plane` crate after #1309 (#1280, #1323); `hosts/host-web/MUTATIONS.md` row `:563` (#1479). By named exception, for #1323 D3's `SuccessorBase::new` only: the `SuccessorBase` construction sites in `crates/host-core/tests/support/successor.rs`, `crates/host-core/tests/withdrawn_successor.rs` (B's, #1344) and `crates/capi/tests/resource_lifecycle.rs` (B's).
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
| 14 | #1479 | Give the spectrum capture collection no cloning accessor | — | — |

## Stream B

- **Coordinator scope:** Control plane and protocol: the portable `control-plane` crate, candidate withdrawal and supersession, scheduled adoption, latest-target cells, path field, watermark and service, typed refusals, seek contract, telemetry defects.
- **Owns:** new `crates/control-plane`, `crates/capi` (incl. `include/miso_engine_v1.h`), `crates/protocol`, `crates/engine/src/realtime`, `crates/host-core/src/live_delta.rs`, `crates/source` (seek parts); by named exception: `crates/graph` route lanes (#1347), the drain signatures in `crates/graph` and `crates/rack` (#1504), `crates/host-core/src/prepare.rs` (#1344), `hosts/host-web` cell and status code (#1312, #1345-#1347, #1349), `crates/effect-compiler/src/prepare.rs` (#1315, #1345), policy and audit scripts each spec lists, `Cargo.lock` (#1447: the `bench-support` dev-dependency line of the `source` package); for #1499: the new `scripts/build-capi-release.sh`, `scripts/check-capi-abi.sh` (its build lines and default library paths; no stream lists it), `scripts/check-release-shape.py` (only if `capi`'s crate-type list changes), `scripts/check-cross-targets.sh` (G's #1472 section: the iOS and Android emission commands only), the `qualification.yml` steps that run the new script and its check, `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` (J's; one owner entry and its case, only if the router refuses the new path), `crates/lane/src/fpenv.rs` (G's; the "Realtime properties" comment lines only), `docs/REALTIME_DEPENDENCY_POLICY.md` (`fpenv.rs` citations only, if lines move), `docs/TARGET_MATRIX.md` (H's; the iOS and Android rows) and one paragraph of `docs/C_ABI_V1_QUALIFICATION.md`. #1345 Amendment 1 needs no new exception: its rack edit is in its existing "`crates/rack/` ... where they ... call `stage`" exception, and its floor edit is the standing exception ("How the streams run").
- **Depends on:** S0; #1344 needs A #1277; #1349 needs H #1381 and #1399.
- **Parallel-safe with:** A, G, J, K, H(a).

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1309 | Extract the C ABI control plane into a portable crate both hosts call | — | — |
| 2 | #1343 | Let the control thread withdraw an unadopted candidate plan | #1309 | — |
| 3 | #1314 | Publish an applied-revision watermark and complete edits asynchronously | #1309, #1343 | — |
| 4 | #1311 | Adopt a successor plan no earlier than a scheduled sample | #1309, #1314, #1343 | — |
| 5 | #1348 | Add miso_engine_v1_service for bounded control work between edits | #1309, #1311, #1314 | — |
| 6 | #1432 | Add the latest-target cell primitive and its loom model | #1309 | — |
| 7 | #1502 | Give each plan its own revision gate and take each block's live snapshot from it | #1309, #1311, #1314, #1343, #1432 | — |
| 8 | #1503 | Bound committed revisions at the plan gate's ceiling | #1432, #1502 | — |
| 9 | #1504 | Hand each block's live snapshot to every live drain | #1502 | — |
| 10 | #1312 | Hold live values in latest-target cells on both hosts | #1309, #1348, #1432, #1502, #1503, #1504 | — |
| 11 | #1447 | Bound the source acquire by the blocks queued at its entry, and observe a late seek outside the loop | #1309, #1343, #1314, #1311, #1348 | — |
| 12 | #1344 | Prepare a successor across a withdrawn candidate plan | — | #1277 |
| 13 | #1318 | Report held source blocks apart from underruns | — | — |
| 14 | #1305 | Find live C ABI edit targets without linear scans | #1309 | — |
| 15 | #1313 | Report each transaction's edit path in its response | #1309 | — |
| 16 | #1315 | Refuse commands that would be acknowledged with no effect | #1309 | — |
| 17 | #1398 | Size the C ABI's plan capacities and resource admission for a superseding candidate | #1309 | — |
| 18 | #1310 | Supersede an unadopted candidate plan by compare-and-swap | #1309, #1314, #1343, #1344, #1398, #1502 | — |
| 19 | #1316 | Anchor every seek on the plan's source-read clock | #1314, #1318 | — |
| 20 | #1317 | Document the seek contract and the C ABI growth rule in the header | #1316, #1318 | — |
| 21 | #1350 | Tighten the seek entry points: source.id.invalid, a typed held preparation, timed reads only | #1316 | — |
| 22 | #1319 | Test held seeks across swaps and supersession, and add a seek to audit capi | #1310, #1348 | — |
| 23 | #1351 | Report each configured counter's own value in the C ABI counter snapshot | #1309, #1312, #1348 | — |
| 24 | #1346 | Hold strip input-lane values in latest-target cells | #1312 | — |
| 25 | #1347 | Hold route-lane values in latest-target cells | #1312 | — |
| 26 | #1399 | Report live_values_superseded in the browser status and prove both hosts drain strip cells alike | #1312 | — |
| 27 | #1505 | Prove under a racing render that a live transaction lands in one block and the watermark names it | #1312 | — |
| 28 | #1352 | Report each configured meter handle's own meter in the C ABI meter batch | #1309, #1351 | — |
| 29 | #1345 | Hold effect parameter, bypass and EQ-target values in latest-target cells | #1312, #1399 | — |
| 30 | #1349 | Publish the applied-revision watermark in the browser status | #1309, #1314, #1348, #1399 | #1381 |
| 31 | #1482 | Attribute each claim's revisions exactly in the watermark and share one seqlock | #1314 | — |
| 32 | #1499 | Build the shipped mobile C ABI libraries with the release profile's fat LTO, and gate it | — | G #1495 |

- **Batches (root, 2026-10-05).** Batch 1 is orders 1-5. Batch 2 is orders 6-10,
  #1432 → #1502 → #1503 → #1504 → #1312, the revision-bounded cells: root's binding requirement
  that one committed revision takes effect in one block and that the watermark's `first_sample`
  is exact. Its design and review are in `revision-bounded-cells/`, and root's five rulings are recorded in #1432 Amendment 1. The rest
  follows in this order, starting with #1447 (order 11), which waits only for batch 1.
- No other slice builds on `store_revision`, `active_revision`, `RevisionTarget` or `set_revision`
  (#1502 deletes them).
- **Watermark bound on `main` today.** Never early. `first_sample` is the start of the first block
  that begins after the commit's last write, so at most one block after the submit returns. A
  live value of that revision can apply earlier, in any block that ran while the submit pushed
  its records. One transaction's values can spread across those blocks until #1502, #1503, #1504,
  #1312 and #1345 have all landed, which together make it exact.

#1447 is stream B batch 1's successor in `crates/source/src/lib.rs` and merges before #1318 (the
hot-file row above). #1345's row does not change; its Amendment 1 adds J #1444 as a dependent.

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
| 9 | #1430 | Measure the link glide for the gate-expander, transient shaper and limiter at the ramp defaults | #1055 | — |

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
- **Owns:** `crates/lane`, `crates/dsp-reference`, effect crates' parameter and designer code, `crates/effect-runtime` (#1366, #1375); by named exception: `crates/builtins` `InputStage::apply_prepared_filter` (#1407), the builtins ramp tests and pins (#1408), the `ParameterSmoother` `Linear` arm in `crates/effect-contract`, the effect payload refusal rows and soft clip overshoot tests it supersedes, the effect corpus pins and the #1301 amendment note (#1409), `ramp_path_within` and its docs in `crates/effect-runtime/src/state_payload.rs`, the effect crates' restore validators' ramp range rules (compressor `state.rs` `validate_channel`, gate `parse_lane`, multiband, delay `read_carried_ramp`, transient shaper and limiter `read_lane`, limiter `coefficient_bounds`, soft clip `ramp_current_valid` and `ulp_at`) with their payload-refusal unit tests and gate-1 rows (`crates/compressor/tests/payload.rs`, `crates/gate-expander/tests/state.rs`, `crates/multiband-compressor/tests/product.rs`, `crates/transient-shaper/tests/contract.rs`, `crates/soft-clip/tests/state_roundtrip.rs`) and `crates/delay/tests/MUTATIONS.md` M18 and M19 (#1411; stream A's payload code and the effect owners'), `crates/parametric-eq` rest predicates (#1328), `crates/builtins-compiler` tail rule (#1329; its tail unit tests, #1433), `crates/graph-compiler` extent (#1379), and for #1379 Amendment 1: `crates/builtins/src/tail.rs`, `crates/builtins/src/lib.rs` (the bound entry points and re-exports), `crates/builtins/tests/tail_contract.rs`, `crates/builtins-compiler/src/lib.rs` (the bound type, the seal, `input_bounds`, the tail-entry charge, their unit tests), `crates/graph/src/lib.rs` (the rename only), the effect crates' `tail_and_rest` functions (the rename and the field only) and their `PreparedEffectMetadata {` literals in payload code and test helpers (the field only) (#1464); `crates/builtins/src/tail.rs` and `crates/builtins/tests/tail_contract.rs` (#1465, #1466, #1467, #1468); `crates/graph/src/lib.rs` (`GraphNode`'s field and the test literals that build one), `crates/host-core/src/prepare.rs` (the tail copy and the gain-liveness selection), every `GraphBuiltinsCompileRequest {` literal (the new field only), `crates/capi/src/runtime/compile.rs` (one read) and `crates/capi/src/abi.rs` (two doc comments) (B's), `crates/capi/tests/tail_every_peak.rs` (new; F's column) and the decision-15 ruling's D15-4(b) and (c) sentences (S0's) (#1379); only if #1468 lands after #1379, the graph pins its tighter value moves (#1468), `crates/graph` per-node effect render node and response row, `crates/graph-compiler/src/estimate.rs` live-control owner charge and its `lib.rs` test mirror (#1460), `crates/host-core/tests/live_delta.rs` rows (#1336, #1337, #1367), `sdk/` (#1369), classifier rows (#1371), `tools/audit/src/vectorization.rs` (the unarmed SVF probes, `execute_probes` and `ACTIVE_REGISTRY`) and `tools/audit/vectorization-allowlist.tsv` (one row) (#1490, after J #1481); for #1494: the write call sites and `allow` attributes of `crates/capi/src/runtime/tests.rs` (B's), `crates/host-core/tests/fp_environment.rs` and `tools/wasm-gates/tests/g6_full_corpus_ftz.rs`, one realtime unsafe-allowlist row per new unsafe file (the awk gate's exclusion line while it is on `main`, then `Policy::workspace()`; J's), one path in `scripts/check-bench-policy.sh`'s `tools/` unsafe set (J's) and the matching entries of `docs/REALTIME_DEPENDENCY_POLICY.md` "Unsafe-code ownership"; for #1495: the comment `crates/capi/src/ffi.rs:813-816` (B's, only if found false) and `docs/REALTIME_DEPENDENCY_POLICY.md`'s `fpenv.rs` citations (only if they move); for #1498 (comment lines only): `crates/host-core/src/render_session.rs:8-9` and `crates/host-core/src/lib.rs:78` (no stream lists either file) and the doc comment `crates/capi/src/runtime/tests.rs:3080-3081` (B's).
- **Depends on:** A's carry slice for each effect crate (#1279, #1280, #1282); #1069 for the multiband; E #1054 for #1371.
- **Parallel-safe with:** A (coordinate on effect crates), B, E, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1328 | Flush the SVF jointly so builtin and EQ filters reach exact rest | — | — |
| 2 | #1451 | Let the builtins splat their chain constants without iOS memset calls | #1328 | — |
| 3 | #1454 | Bring the builtins dual loop back to P0's instruction count | #1451 | — |
| 4 | #1455 | Give the multiband compressor an unarmed form for live audio | #1451 | — |
| 5 | #1336 | Make the gate-expander's attack, hold and release live | — | #1279, #1280 |
| 6 | #1337 | Make a parametric EQ band's enabled and kind live | — | #1279, #1280 |
| 7 | #1366 | Prove the crossover designer total and share the SVF ramp stability check in effect-runtime | — | — |
| 8 | #1339 | Give the delay a live bypass shunt | — | #1282, #1315, #1341 |
| 9 | #1368 | Lower the link mode to per-lane state in the linked effects' banks | — | #1279, #1280 |
| 10 | #1407 | Retarget a live input filter only through its designs and their mixtures | — | — |
| 11 | #1428 | Run the release reachable-word sweep of the live input filter in required CI | #1407 | — |
| 12 | #1408 | Keep every trim, fader and matrix ramp inside its endpoints | — | — |
| 13 | #1409 | Keep every effect parameter ramp inside its endpoints | #1408 | #1301 |
| 14 | #1411 | Remove the 64-ulp restore slack once every effect ramp is clamped | #1409 | #1301 |
| 15 | #1458 | Share one effect-ramp endpoint harness instead of seven copies | #1409, #1411 | — |
| 16 | #1459 | Replay an effect ramp whose endpoint clamp acts in the wasm corpus | #1409 | — |
| 17 | #1473 | Pin ramp_toward's operand order, signed zero and NaN in the wasm corpus | #1459, #1455 | — |
| 18 | #1452 | Undo the iOS memset ratchet workarounds once splats are free | #1451, #1409 | — |
| 19 | #1456 | Remove the libc memset calls from the true-peak limiter's reset on Apple targets | #1451, #1452 | — |
| 20 | #1472 | Measure the iOS memset_pattern16 ratchet on the shipped post-LTO capi staticlib | #1456 | — |
| 21 | #1329 | State a bounded tail and an exact-rest bound for every node | #1328, #1407, #1408 | — |
| 22 | #1433 | Tighten the cascade exact-rest bound with a frequency-aware cascade analysis | #1329 | — |
| 23 | #1457 | Cache design bounds across preparations within a stated preparation budget | #1329 | — (the C ABI cache is #1471, the browser cache #1470) |
| 24 | #1474 | Remove the data-dependent cost of the near-top tail walk | #1457 | — |
| 25 | #1471 | Wire the engine and session design-bound caches into the C ABI | #1457 | #1309 |
| 26 | #1470 | Cache design bounds across browser preparations in the control Worker | #1457 | #1332 |
| 27 | #1338 | Make the multiband compressor's crossover live | #1366 | #1069, #1280, #1282 |
| 28 | #1340 | Give the multiband compressor a live bypass shunt | #1339 | #1069, #1280, #1282, #1315, #1341 |
| 29 | #1369 | Declare a strip's console link mode in the session, the wire and the SDK | #1368 | — |
| 30 | #1370 | Ramp a lane's detector link between modes | #1368 | — |
| 31 | #1460 | Keep only render-read effect fields in the render node table | — | — |
| 32 | #1377 | Carry each effect's tail and exact-rest bound in its prepared metadata | #1329, #1460 | — |
| 33 | #1461 | Keep each effect processor's render memory free of its prepared metadata | #1377 | — |
| 34 | #1462 | Compute and validate each effect's tail bound once per rate and quality at registry build | #1377 | — |
| 35 | #1469 | Build the launch effect registry once per process and share it | #1462 | — |
| 36 | #1464 | Carry every node's tail bound in one node-neutral struct | #1457, #1461, #1462 | — |
| 37 | #1465 | State a fixed input section's decay, gains and flush stall | #1457, #1464, #1474 | — |
| 38 | #1484 | Split a node's flush stall into a peak stall and a tail stall | #1464, #1465 | — |
| 39 | #1466 | Certify the live input section's decay, peak gain and flush stall | #1457, #1465, #1484 | — |
| 40 | #1467 | Derive the live input section's tail gain and state its composition | #1466 | — |
| 41 | #1371 | Carry the link record from the edit to the lane | #1369, #1370 | #1054, #1279, #1280, #1312, #1345, #1364, #1394 |
| 42 | #1379 | Define how node tails compose through gain in the graph extent | #1329, #1377, #1464, #1465, #1466, #1467, #1484 | #1237, #1285, #1384, #1287 (first slice) |
| 43 | #1468 | Tighten the fixed input section's peak gain past the cascade triangle inequality | #1465, #1467 | — |
| 44 | #1485 | Tighten the live input section's peak gain toward the real-kernel history peak | #1466, #1467, #1484 | — |
| 45 | #1367 | Make the multiband compressor's link mode live | #1371 | #1069, #1280, #1282 |
| 46 | #1372 | State the parametric EQ's bounded tail and exact-rest bound | #1328, #1329, #1377, #1379, #1461, #1462, #1464, #1469 | — |
| 47 | #1375 | Report a zero tail beyond latency for the compressor and the true-peak limiter | #1377, #1461, #1462, #1464, #1465, #1469 | — |
| 48 | #1236 | Let a strip override a console slot's link mode | #1367, #1368, #1369, #1370, #1371 | #1054, #1196, #1279, #1280, #1345, #1394 |
| 49 | #1373 | State the multiband compressor's bounded tail and exact-rest bound | #1329, #1338, #1375, #1377, #1379, #1461, #1462, #1464, #1469 | — |
| 50 | #1374 | State the delay's bounded tail and exact-rest bound | #1375, #1377, #1379, #1461, #1462, #1464, #1469 | — |
| 51 | #1376 | State exact-rest bounds for the gate, transient shaper and soft clip | #1375, #1377, #1461, #1462, #1464, #1465, #1469 | — |
| 52 | #1378 | Retire the Infinite tail | #1372, #1373, #1374, #1375, #1376, #1379, #1469 | — |
| 53 | #1487 | Diagnose and tighten the live input section's remaining peak and tail gain looseness | #1485, #1379 (low priority) | — |
| 54 | #1490 | Probe the unarmed SVF block form in the nightly vectorization report | — | J #1481 |
| 55 | #1494 | Make the floating-point control-word writers sound by construction | — | — |
| 56 | #1495 | Prove or correct fpenv's realtime-cost and DAW-callback claims against the shipped x86_64 codegen | — | — |
| 57 | #1498 | Narrow the universal DAW-callback FTZ/DAZ claim in host-core and the C ABI tests | #1495 | — |

## Stream H

- **Coordinator scope:** Browser control plane: shared-memory spike, allocation gates, nightly browser artifact, Worker control plane, render-only worklet, transaction API in the SDK.
- **Owns:** `hosts/host-web`, `sdk/`, `scripts/check-web-audioworklet*`; #1334 also `rust-toolchain.toml`, `.github/workflows/{qualification,npm-publish}.yml`, `docs/RELEASE.md`, `docs/TARGET_MATRIX.md` and the build/identity scripts it lists; #1333 also `tools/parameter-metadata/src/abi_layout.rs`, `scripts/check-abi-layout-v1.py`; #1381 the `RuntimePreparer` hook in `crates/control-plane`; by named exception: `crates/host-core/src/spectrum.rs` (#1449: the `SpectrumCapture` drains, the new `SpectrumCapture::reset_after_cancel`, `SpectrumCaptureCollection::cancel`, `select` and the new `cancel_except`, the new `SPECTRUM_RESULT_SLOTS`, `spectrum_capture_resources_for_id_bytes` (`:331-333`), the capture queue's construction (`:1353`), and their unit tests). #1448 edits only H's own files and no floor; its markers are J's guard commit (stream J). #1477 also edits `scripts/test-web-audioworklet.mjs` (`testQualificationBoot`, the fake port's `miso.renderallocations.v1` reply and the reply-check cases only). #1488 also edits `docs/REALTIME_DEPENDENCY_POLICY.md`, one sentence, only if J #1489 landed first with its D-M2 exception clause. #1491 also edits `scripts/test-web-audioworklet.mjs` (`testQualificationBoot` only). #1492 also edits, by named exception, `crates/host-core/src/spectrum.rs` (`SpectrumTargetRef`, `SpectrumTarget::as_ref`, `select`, `selection_would_change`, `owned_entry`'s removal, the `entry` doc and the unit-test call sites, outside every marked region), `crates/host-core/src/lib.rs` (the re-export only) and `crates/host-core/tests/spectrum.rs` (the `select` call sites only).
- **Depends on:** B #1309, #1312-#1316, #1348, #1349; A #1277, #1327; D #1326; E #1054, #1364; F #1225, #1226, #1247, #1261, #1262, #1267; I #1335.
- **Parallel-safe with:** A, G, J.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1449 | Bound every drain the AudioWorklet runs on its audio thread | — | — |
| 2 | #1448 | Bound the browser meter poll by each queue's count at entry | — | — |
| 3 | #1401 | Prepare every browser preparation branch concurrently, as a successor too, in host-core | — | #1326 |
| 4 | #1333 | Gate AudioWorklet render against allocation statically and at runtime | — | — |
| 5 | #1383 | Build and encode session transactions in the SDK | — | #1394 |
| 6 | #1331 | Prove two Wasm instances on one shared memory in three browser engines and on iOS | — | — |
| 7 | #1400 | Prepare through an adapter-supplied preparer in the control-plane crate | #1401 | #1309, #1326 |
| 8 | #1385 | Encode the session, submix, output, route, automation and VCA edits in the SDK | #1383 | #1335, #1394 |
| 9 | #1334 | Build the browser artifact on a pinned nightly toolchain | #1331 | — |
| 10 | #1380 | Ship the browser module with one imported shared memory at every instantiation site | #1331, #1333, #1334 | — |
| 11 | #1332 | Run the browser control plane in a Worker and keep the AudioWorklet render-only | #1331, #1333, #1334, #1380 | #1057 |
| 12 | #1387 | Move browser source submission and seeks into the Worker | #1332 | #1316, #1318 |
| 13 | #1381 | Swap and retire browser plans through the Worker's service loop | #1332, #1387, #1400 | #1309, #1314, #1327, #1348, #1395 |
| 14 | #1382 | Admit browser live edits in the Worker through the committed model | #1332, #1381 | #1054, #1057, #1225, #1226, #1247, #1261, #1262, #1267, #1312, #1313, #1345, #1346, #1347, #1364, #1390, #1394 |
| 15 | #1290 | Replace the running browser session in the Rust host | #1332, #1381, #1382, #1387, #1400, #1401 | #1277, #1309, #1310, #1313, #1314, #1326, #1327, #1348, #1349, #1395 |
| 16 | #1293 | Export transaction apply and anchored seek from the browser engine module | #1290, #1332, #1381, #1387 | #1309, #1313, #1316, #1319 |
| 17 | #1342 | Make a send's follows_mute live in the browser | #1290, #1382 | #1226, #1313, #1347 |
| 18 | #1386 | Diff a replacement document against the committed model and export replace from the browser engine module | #1290, #1293 | — |
| 19 | #1294 | Send a session transaction to the browser control plane | #1293, #1332, #1381, #1382, #1386, #1387 | #1310, #1348, #1349 |
| 20 | #1295 | Qualify a structural browser edit in real browsers | #1290, #1294, #1332, #1333, #1386 | — |
| 21 | #1296 | Apply session transactions from the browser SDK | #1294, #1295, #1382, #1383, #1385, #1386 | #1312, #1313, #1314, #1325, #1326, #1349 |
| 22 | #1297 | Feed and retire the sources a browser edit adds or removes | #1293, #1296, #1332, #1381, #1387 | #1316, #1325 |
| 23 | #1389 | Apply session transactions from the headless SDK engine | #1293, #1296, #1332, #1381, #1382, #1383, #1385, #1386 | — |
| 24 | #1476 | Read every SDK qualification instance's render allocation count before it closes | — | — |
| 25 | #1477 | Witness the staging-read boot caller and test the render allocation reply check hermetically | — | — |
| 26 | #1488 | Bring the PCM-feed worklet's source submit and seek under the render-locked allocation count | — | — |
| 27 | #1491 | Make the boot contract red when the worklet skips spectrum collection staging | #1477 | — |
| 28 | #1492 | Select a spectrum collection entry without allocating on the browser audio thread | #1488 | A #1479 |

#1448's markers are not in its row: stream J lands them as "H #1448's guard, landed by J after C2"
(stream J's table).

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
- **Owns:** the scripts, tests and files each issue lists; `crates/effect-contract` (#1330); `crates/graph-compiler` and `crates/graph/src/program.rs` (#1384, after #1285); `tools/realtime-policy` (#1438-#1446), `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh` (until #1446 deletes them), and the new `scripts/operator/sync-spec-bodies.sh` and `scripts/operator/test-sync-spec-bodies.sh` (#1445). By named exception:
  - `sdk/test/enginectl-cli.mjs` (#1435, D1's cleanup only; root approved, 2026-10-05);
  - `crates/builtins-compiler/src/lib.rs` (#1441's two markers; #1443's markers and its
    restructure of the test-support `LiveControlInputProcessor` only);
  - `crates/source/src/lib.rs` and `crates/host-core/src/spectrum.rs` (#1443's markers only; the
    rack bank loop is marked by B #1345, not by J);
  - `hosts/host-web/src/lib.rs` (stream H's): the two `poll_meters` markers of "H #1448's guard,
    landed by J after C2" only, each replacing a blank line (#1443's "#1448 guard" section; root
    ruling 2 on the fourth review, option (C));
  - `scripts/check-realtime-policy.sh` (the region floor line) and `scripts/test-realtime-policy.sh`
    (the pad loop, its comment and the region-floor message) for #1441, until #1446 deletes both;
  - `Cargo.toml` (one `members` entry) and `Cargo.lock` (the new package's entry) (#1438);
  - `.github/workflows/qualification.yml` (#1438: `audit-native`'s build flag and one step; #1446:
    `lint`'s two lines and the step's name);
  - `scripts/check-ci-path-routing.py` and `scripts/test-ci-path-routing.py` (#1446:
    `DEDUPLICATED_OWNERS`' entry and its case);
  - comment lines only, each keeping its line count (#1446): `crates/engine/src/realtime/observe.rs`
    and `crates/engine/src/realtime/watermark.rs` (stream B's), `crates/lane/src/fpenv.rs` and
    `crates/lane/src/softfma.rs` (stream G's), `crates/capi/tests/resource_lifecycle.rs` (stream
    F's: F owns `crates/capi` tests), `crates/host-core/src/lib.rs`, `tools/bench-support/src/lib.rs`,
    `scripts/check-bench-policy.sh`;
  - `docs/REALTIME_DEPENDENCY_POLICY.md`, `docs/REALTIME_MEMORY.md` (#1446's lines only) and
    `scripts/operator/README.md` (#1445's sentence and entry);
  - `.github/ISSUE_SPECS/*.md` (#1446's D5 substitutions; S0's files) and
    `.github/ISSUE_SPECS/948-*.md` (#1439's Evidence section);
  - `hosts/host-web/qualification/sdk-response-entry.ts` (`runContinuousSpectrumQualification`
    and `createContinuousSpectrumBrowser`), `hosts/host-web/qualification/run.mjs` (the
    `sdk-spectrum-continuous` predicates) and `hosts/host-web/MUTATIONS.md` (its rows) (stream H's;
    #1480);
  - `crates/lane/src/softfma.rs` (stream G's): comment and `SAFETY` lines only (#1489);
  - `scripts/test-web-audioworklet.mjs` (`testProcessor`'s recording proxy and its check) and
    `sdk/test/browser-pcm-evals.mjs` (the feed worklet harness's recording proxy and its check)
    (stream H's; #1496);
  - `.github/workflows/nightly.yml` (#1481: the `native-vectorization-report` step's
    `continue-on-error` line and its comment, and the file's header bullet for that job) and `scripts/test-test-support-ci.py` (#1481: two
    rewrapped lines).
- **Depends on:** S0; #1303 and #1384 need A (#1277, #1285); the tool batch (#1445 to #1444) needs stream B batch 1 (#1309, #1343, #1314, #1311, #1348), B #1447, H #1448, H #1449 and B #1345 (Amendment 1) on `main`; #1418 and #1426 need the tool batch.
- **Parallel-safe with:** all, except the tool batch's waits above and the hot-file rows for its marker commits.

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
| 15 | #1419 | Use the shared StopOnDrop in the C ABI plan-swap race test | — | — |
| 16 | #1420 | Make the nonadjacent split-pair harness's track choices observable to its tests | — | — |
| 17 | #1421 | Remove both temporary directories the web AudioWorklet test script creates | — | — |
| 18 | #1422 | Run doctests in CI | — | — |
| 19 | #1416 | Let only host-core build the live route records that hosts push | #1422 | — |
| 20 | #1423 | Make live strip records valid by construction | #1422 | #1312, #1346 |
| 21 | #1429 | Fail the web AudioWorklet test step when it leaves anything in its temporary directory | #1421 | — |
| 22 | #1434 | Hold every block-form owner step in qualification.yml to its guard lines | #1429 | — |
| 23 | #1435 | Remove every temporary directory the enginectl CLI test creates, and fail the SDK qualify step on a leftover | #1429 | — |
| 24 | #1445 | Sync edited issue specs to their GitHub bodies with a checked operator script | — | — |
| 25 | #1438 | Check realtime regions with a Rust syntax-tree tool: the crate, the walk, unsafe ownership, markers and floors (D5 amended, root 2026-10-08: the four stale allowlist entries and the wasm-gate-guest entry are dropped from both gates) | — | #1309, #1343, #1314, #1311, #1348 |
| 26 | #1439 | Check realtime regions with a Rust syntax-tree tool: region boundaries, the forbidden-body predicate and the whole-plan check | #1438 | — |
| 27 | #1440 | Check realtime regions with a Rust syntax-tree tool: the drain-bound rule's mapping, scope and innermost bound | #1439 | — |
| 28 | #1441 | Check realtime regions with a Rust syntax-tree tool: the drain-bound rule's outer loops, attributes and constructors | #1440 | — |
| 29 | #1442 | Check realtime regions with a Rust syntax-tree tool: refuse pops in closures, macros and function values, and commit the probe corpus | #1441 | — |
| 30 | #1446 | Check realtime regions with a Rust syntax-tree tool: delete the awk gate | #1442, #1445 | — |
| 31 | H #1448's guard, landed by J after C2 | (the first commit of #1443, not an issue: the two `poll_meters` markers; spec in #1443's "#1448 guard" section) | #1446 | H #1448 |
| 32 | #1443 | Check realtime regions with a Rust syntax-tree tool: put every non-test pop in a marked region or on the control-side allowlist | H #1448's guard | #1447, #1448, #1449 |
| 33 | #1444 | Check realtime regions with a Rust syntax-tree tool: refuse a call to a popping function in a marked loop or closure | #1443 | #1345 |
| 34 | #1418 | Require every loop around a realtime drain to drain a different queue on each pass | #1444 (the whole tool batch on `main`) | — |
| 35 | #1426 | Bound each realtime drain per queue: require the pop receiver to be the counted queue | #1418 | — |
| 36 | ~~#1478~~ | ~~Name every approved unsafe file in the realtime dependency policy~~ (superseded by #1489, root 2026-10-08; closed, both doc commits reverted) | — | — |
| 37 | #1480 | Make the continuous-spectrum gate's capture loss independent of browser timing | #1248 | — |
| 38 | #1481 | Make the nightly vectorization report pass on main and report its failures | — | — |
| 39 | #1489 | Correct the realtime dependency policy's unsafe-ownership statements | — | H #1488 (preferred; else its D-M2 exception clause) |
| 40 | #1496 | Prove that every export the worklets call after construction is render-locked | #1446 | H #1488, H #1492 |

**#1444 waits for B #1345 (Amendment 1): accepted by root, 2026-10-05**, in place of the earlier
"not ordered against #1345" for this one slice.

**C2 (#1446) before B2b-1 (#1443), and the #1448 guard between them (root ruling 2 on the fourth
review, option (C)).** H #1448 lands its code unmarked, which is no worse than `main` today. The awk
gate does not judge an unmarked function, and the per-queue shape is the tool's (#1440's D8b), so
the guard commit comes after #1446 deletes the awk gate. #1443 requires every render-thread pop to
be marked, so it comes after the guard. #1438 to #1442 cover every check of the awk gate, so #1446
needs nothing from #1443 (fourth review, BLOCKER-1).

**Delivery (root, 2026-10-05):** the nine-slice plan (#1445, #1438, #1439, #1440, #1441, #1442,
#1446, #1443, #1444) is accepted, and the whole set lands in one J batch with the #1448 guard
commit: local checkpoint commits per slice, one push at the batch boundary. The batch boundary
therefore follows #1447, #1448, #1449 and #1345 (Amendment 1) on `main`.
- **#1418 and #1426 are outside the batch** (root, 2026-10-05). They start when the batch is on
  `main`, so they come after #1444 and after #1345.
- **Cost.** All nine tool issues stay unpushed until #1345 lands. On stream B's table after batch
  1, #1345 is row 29, behind #1432, #1502, #1503, #1504, #1312 and #1399.

## Stream K

- **Coordinator scope:** Research and design notes.
- **Owns:** `docs/handoffs/`.
- **Depends on:** none
- **Parallel-safe with:** all.

| Order | Issue | Title | After (same stream) | After (other streams) |
|---|---|---|---|---|
| 1 | #1057 | Design: one edit API on every host over the core's committed session model | — | — |
| 2 | #1058 | Research: render stored session automation in the engine, identically on every platform | — | — |
