# RT-1 — scatter full, unfolded banks directly to distinct planar destinations

Astra-approved scope, 2026-09-05, inspected main `1ef2375c0a9fa4b1481e844a1f68f48d22f8dc6f`. This complete contract is issue #399. Root freezes the actual post-integration implementation base before assignment. No implementation or measurement was performed for this brief.

## Readiness and smallest closable outcome

The premise is still present: `crates/rack/src/lib.rs::BankChain::scatter_tiled` calls `tile_scatter` into both full-block staging buffers, then copies each staged lane to `BankMembers::plane_mut`. The non-fold arm therefore materializes each output twice. `ArenaMembers` still exposes only one mutable stereo lane at a time. This is independent of accepted-automation delivery and of #306; no automation prerequisite applies. Prefer landing the in-progress #371 marker fix first to qualify this new render code under its expanded gate. Native AArch64 stays deferred under #378.

Implement a direct tiled scatter for full banks whose member provider can supply simultaneous, pairwise-disjoint destinations. Preserve the existing staged fallback for unsupported providers and every folded chain; preserve the partial-bank path. This closes RT-1's redundant render traffic on its production eligible path. RT-2's route accumulation and RT-11's by-value transpose representation remain separate.

**Root-approved scope decision (2026-09-05) to put in the numbered issue:** retain existing preallocated staging storage in this slice, including full unfolded chains. The proposed optional runtime accessor can decline; `BankChain::new` does not know its later provider type. Removing all fallback storage would require a separate prepare-time capability contract or sacrificing direct SIMD fallback. Do not pretend this implementation removes the audit's separately projected allocation bytes. If removal of unused retained capacity is desired, record it as a stateless successor; do not silently delete storage then allocate upon a render-time refusal. This is a memory-retention limitation, not a remaining extra render copy on the successful direct path.

## Permitted implementation

- `crates/rack/src/lib.rs`: optional safe `BankMembers` multi-plane borrowing seam, default unsupported; a direct tile helper using existing `transpose_tile_4/8`; direct ragged tail; focused tests and accurate comments. Use fixed stack storage bounded by bank width 4/8. Keep exactly one gather/scatter counter event per run.
- `crates/graph/src/runtime.rs`: implement the seam for `ArenaMembers` using its actual scatter `outputs` (never gather `inputs`), preserving redirected destinations. Do not change bind, cohort membership, folding, mono, effect, routing, observation or arithmetic decisions.
- `crates/engine/src/realtime/disjoint.rs`: smallest sound mutable-set borrowing primitive beside `write_stereo`, with lifetime tied to the lease and existing I1–I4 rationale extended to all returned lanes/planes. Unsafe remains in this existing allowlisted file.
- Numbered issue, dedicated evidence README/output directory, and one new matching arm in `scripts/run-console-benchmark.sh` and `scripts/operator/preflight-console-benchmark.sh`, including both usage lists. No generic runner framework or validator/schema changes.

Accessor contract: success supplies exactly one complete L/R view per requested lane in stable lane order; views are mutually disjoint, correctly sized and live only under the exclusive lease borrow. Failure supplies no usable views, performs no PCM writes, and permits the original fallback. Release safety cannot rest on a duplicate-ID `debug_assert`: a safe method accepting arbitrary IDs must reject duplicates, unauthorized/out-of-bounds IDs and mismatched output capacities **before creating any references**, or use an actually enforced prepared capability that makes these inputs unrepresentable. A bounded O(W²) check at W≤8 is acceptable; never scan all tracks per scatter. Do not create aliased references then discover the duplicate. No unchecked unwrap/panic as the normal failure arm. A borrow checker obstacle is reason to simplify the seam, not add unsafe in rack/graph.

Class A is exact word permutation: no FP arithmetic, rounding, NaN canonicalization, denormal flush, reassociation or FMA changes. Fold hooks must still receive each entire lane block exactly once in the original lane order. Extending direct scatter to folded destinations or combining route reductions is outside this brief. No class B owner ruling is needed within these limits; changing output bits/order would violate scope rather than become an accepted tolerance.

## Objective gates

1. Baseline and candidate `cargo test --locked --workspace`, serialized in one target; record pass/fail/ignored counts and intentional deltas. Focused iteration: `cargo test --locked -p engine -p rack -p graph`; final `cargo fmt --all -- --check` and existing workspace clippy form (also included by preflight).
2. Extend the existing `full_bank_gather_scatter_round_trip_is_bit_exact` and `tiled_transpose_matches_the_scalar_path_bit_for_bit` coverage so the direct arm is demonstrably exercised, not just default unsupported fallback. Compare direct, staged fallback and forced scalar with `to_bits`, using both widths and existing hostile corpus/frame_shapes: signed zeros, subnormals, infinities, NaN payloads, exact multiples, fewer-than-width and ragged frames. Keep IndexXor-style nonidentity resident layout verification. Include repeated blocks. Assert direct-success never calls per-lane fallback/copy and the staging sentinel remains unchanged; this proves the traffic claim more directly than a timing improvement.
3. Test accessor soundness at the public boundary: duplicate and invalid/not-writable IDs, malformed capacities, success writes isolated L/R destinations and preserves all nonselected buffers. Prove rejection is nonpartial and works in release (`cargo test --locked --release -p engine` with precise new test filter is sufficient). Test an unsupported provider still uses fallback. If using a prepared capability instead, test invalid capability construction before render and the equivalent exclusion proof.
4. Existing partial/inactive-lane identity, fold identity, mono/collapse, auxiliary destination and graph redirect/PDC/deterministic identity suites remain green. Add a representative real `ArenaMembers` graph fixture exercising the direct path with different input/output IDs and compare an independent staged/scalar reference. Assert chain/transpose/fold counters unchanged; don't add permanent product telemetry solely to count a test branch.
5. `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`, `bash scripts/check-rack-policy.sh`, workspace policy and existing engine/graph allocation probes. Prove repeated direct render calls allocate/free zero after prepare (all test setup and buffer borrowing scaffolding excluded from measured scope). Preserve #371's regions. Compile/check the supported wasm32 target using the repository's pinned SIMD flags and existing host-web build gate; native AArch64 is not a gate. The complete qualification CI remains required before merge.
6. Review source/generated optimized code or a small captured release assembly excerpt to establish direct successful path lacks full-block staging writes/copy. Do not require an exact instruction count or manufacture an additional benchmark framework. Fixed tile temporaries/register spills are not a claim of literal zero machine stack traffic.

## Frozen descriptive measurement

Register `--issue399-rt1` identically in runner/preflight, writing a fresh `artifacts/issue399-rt1` directory. Preserve existing #368/#388 arms after merging. Do not edit workload, fixtures, floor tables or production validators to improve the number. Root freezes source, arm and validators in a committed checkpoint after semantic PASS; only then run:

```
PATH=/home/bl/.cargo/bin:$PATH bash scripts/operator/preflight-console-benchmark.sh --issue399-rt1
PATH=/home/bl/.cargo/bin:$PATH bash scripts/run-console-benchmark.sh --issue399-rt1
```

The first command is non-timed and must PASS before the second. Persist its stdout/stderr outside the directory whose existence the runner refuses. It must prove arguments, schema, output creation/persistence, explicit failure propagation, overwrite refusal and zero workload launches, using the existing hermetic validator/precondition suites. Preflight and runner use different existing release profile settings; document actual runner binary digest/profile separately instead of falsely equating their hashes.

Exactly one runner invocation, one warmup, two measured rounds, full existing 46-record corpus. No before-run timing, retiming, tuning or second fixture benchmark. Validate every current record and the complete aggregate with the unchanged production validators; verify raw/accepted byte counts/hashes, candidate commit, runner binary and disposition. Preserve all raw output/stderr on any post-launch tooling failure and move repair/promotion to a tooling successor; don't rerun.

Record the idle row and existing console/ragged/mono counters/digests. Compare output digests and structural counters to the latest applicable sealed same-workload evidence (currently #368) and/or non-timed baseline identity proofs; compare numerical timings only with explicit host/workload comparability limitations. The historical `artifacts/issue184` idle value 12.978 is historical context, not an acceptance threshold. Missing cycle fields remain null; do not synthesize cycles from wall time or the floor denominator. Class-A floors do not change. Explicitly include the bounded release access-validation overhead in the descriptive interpretation; do not remove safety checks or chase performance retries. Folded console rows may see no benefit, which is honest: report eligibility and actual result, no promised speedup. Measurement is descriptive; identity, soundness and removal of the redundant copy are the gates.

## Delivery and attempt budget

Astra briefs; Luna one coherent implementation attempt; Astra adversarial checkpoint review. If insufficient, Sol receives a bounded revision against explicit failures, with at most three attempts total under repository rules. Root owns exact-path checkpoints, pushes and GitHub synchronization. After failure at attempt three, preserve evidence and rescope; no fourth retry. Root authorizes the sole timed command only after checkpoint PASS and non-timed gates. Astra reviews the final actual PR at its exact pushed head, including evidence and integration artifacts; root merges only after PASS and required CI. Benchmark/body claims must distinguish direct eligible path, retained fallback memory and unchanged folded path. Automation delivery and RT-2/RT-11 remain independent findings.

## Root delivery decision

The frozen implementation base is post-#371/#388 main `0c2b283f86b199351b78be99784def7c614c0320`. Astra approved proceeding independently of the remaining #306 tooling family, while #400 has only immutable qualification outstanding. Astra's retained-storage and release-safe borrowing scope is accepted. The direct path claim is reduced render memory traffic, not reduced retained plan memory.

If the actual changed Rust source invalidates the shipped artifact, the bounded delivery scope also permits the existing source digest pin, `.github/workflows/npm-publish.yml` expected digest, current artifact identity prose in `docs/C_ABI_V1_QUALIFICATION.md`, and generated `hosts/host-web/qualification/results.json` / `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`. Qualify the actual reproducible artifact from an immutable source candidate through the existing three-browser/static/resource pipeline. This permission does not authorize a new qualification framework, publication, additional benchmark runs or rewriting historical measurements.

## Astra sequencing amendment — 2026-09-05

# Astra #399 sequencing decision — APPROVED

#399 RT-1 may begin as the single active launch-critical feature in a fresh isolated worktree while immutable #400 tooling qualification completes. #371's expanded realtime gate and #388 are merged; no RT-1 source, safety or evidence contract depends on completion of #306 or its remaining children. The earlier Wave-0 waiting paragraph is a sequencing preference, not a technical prerequisite, and root may explicitly amend/synchronize it before assigning Luna.

Amendment: “After merged #371/#388, #399 may run independently of the remaining #306 tooling family. Keep one launch-critical feature implementation tranche, preserve isolated worktrees and existing history, and serialize any shared-target Cargo/artifact operations. Independent tooling qualification and read-only briefing/review may proceed. Merge current main and resolve actual integration conflicts before final PR review; do not assume an earlier gate result qualifies a changed head.”

All frozen #399 scope and gates remain unchanged: safe release disjointness checks, exact rendered-word identity, retained fallback allocation, unchanged folded/partial behavior, applicable artifact/publisher/browser identity qualification, and precisely one descriptive benchmark after committed-source preflight and Astra checkpoint PASS. Run that timing only when concurrent qualification/build load is settled; do not consume it amid avoidable overlapping builds or retry it afterward. No new performance claim or skipped safety gate follows from this sequencing amendment.

Root updates the numbered local/remote spec before Luna attempt 1, owns checkpoints/pushes and respects Astra review plus required CI before merge. #401 remains planning-only until #400 merges and the final helper API is frozen. No implementation or expanded scope was authorized by this decision beyond the existing #399 contract.


Root normally merged the frozen current main into the isolated #399 brief branch before assignment. Luna owns attempt 1; root requires a focused-green source checkpoint and Astra semantic PASS before the sole benchmark preflight/invocation. No timing is authorized at initial implementation. #399 is the only active launch-critical feature implementation; tool migrations remain independently isolated.

## Luna attempt 1 checkpoint — not accepted

Luna implemented the optional multi-plane provider seam, arena prevalidation and direct scatter. Root explicitly permits the necessary `crates/engine/src/realtime/mod.rs` re-export of the new writable-plane type, in addition to the originally listed implementation paths; no further scope expansion is implied.

Focused Cargo tests passed: rack 34 tests; graph/engine reported 82 plus doctests, with formatting PASS. Logs: `/tmp/engine-rt1-rack-direct.log`, `/tmp/engine-rt1-graph-engine.log`, `/tmp/engine-rt1-focused.log`, `/tmp/engine-rt1-fmt-final.log`. This is a buildable checkpoint, not acceptance. Root's required realtime-policy check FAILS on three new `expect` calls in marked rack render code (`/tmp/engine-399-luna-realtime.log`). Required directed rejection/release/accessor soundness and staging-sentinel/real-graph identity evidence are not yet supplied. Root also flags the right-plane loop performing the full tile transpose once per lane for review against the intended direct tiled path.

No benchmark arm, timing, artifact build or full-workspace qualification was performed. Astra must provide the attempt verdict before any further implementation; on FAIL, Sol receives the next bounded attempt. No realtime gate or safety contract may be weakened to accept this checkpoint.

## Astra attempt 1 verdict — FAIL

# Astra #399 RT-1 attempt 1 review

**FAIL — bounded Sol revision required at exact pushed `c9595d3714e679b4ca525b1cb99dd4ff99496a7d`.** Luna attempt 1 is consumed; no timing, artifact qualification or full-workspace promotion is authorized from this checkpoint.

## Blocking findings

1. **Unsafe out-of-allocation slice construction in the new safe arena API.** `ArenaLeaseSetBuilder::new` accepts any nonzero plane count, including one. `write_stereo_many` checks width, frames and writable/distinct buffer IDs but never checks `self.arena.planes >= 2`. Its right-plane offset assumes two planes, then uses raw pointer offset/dereference and from_raw_parts_mut; a valid one-plane arena with valid writable IDs can reach an out-of-allocation pointer. Add release validation of plane availability before forming any reference. Document spatial range/lifetime and I1–I4 reasoning, using construction size bounds for offset safety. Do not assume existing write_stereo's caller convention makes this new checked API sound. This finding follows directly from the public constructor and pointer expressions; no undefined-behavior probe was executed.

2. **The new safe provider/view shape is insufficiently validated.** `BankPlaneViews::from_pairs` accepts either width but records neither the requested width nor frame capacity; a safe provider can return four pairs when eight were requested, or slices shorter than frames. scatter then reaches missing Option expects or slice bounds, potentially after writing the left plane/earlier lanes. Validate the complete returned shape before any PCM write, or encode width/complete capacity in a type that cannot represent a mismatched success. Refusal must drop all borrowed views cleanly and use the existing staged fallback. Do not replace expects with unchecked access, partial skipping or early success. Safe Rust already establishes disjointness of supplied &mut slices; the unsafe arena producer separately owes its checked ID proof. `ArenaMembers` also slices `outputs[..4/8]` before checking length; use a checked length/shape path rather than expecting try_into().ok() to protect the earlier slice.

3. **Realtime gate red.** Independently reran check-realtime-policy.sh: it rejects the three new `.expect("direct destination present")` calls in marked rack code at approximately lines 326, 334 and 2153. The marker region and ban must remain intact. Fix the representation/control flow so the valid render path has no introduced panic surfaces. Test-only expects outside render are a separate matter.

4. **Right-plane direct scatter repeats the full transpose per lane.** The lane-outer right loop reconstructs and transposes the same W×W tile W times, whereas left transposes once and stores all lanes. This does not implement the frozen one-tile transpose/direct-destination mechanism on both planes; it substitutes repeated loads/shuffles for the copy being removed. Use one transpose per tile per plane and write each resulting row to its lane, plus direct ragged tails. Retain existing transpose kernels and exact bits, with no RT-11 representation project. Remove the inaccurate comment that reborrowing right views is impossible. A test callback counter or focused code inspection should establish this bounded operation structure without benchmarking.

5. **Assigned acceptance evidence is materially incomplete.** The only rack test change equips the existing Planes provider, so existing identity tests now exercise direct vs scalar, but there is no independent staged-fallback comparison, no complete new API rejection suite, no staging/call sentinel, no real graph redirect proof and no repeated-call allocation proof. The frozen spec explicitly required these because this slice adds unsafe multi-borrowing and changes the render path. Focused old-test success alone is insufficient.

## Sol attempt 2 — one bounded coherent pass

Stay in the approved engine/disjoint, minimal engine/realtime/mod.rs re-export, rack and graph paths plus their focused tests and #399 evidence. Retain preallocated fallback storage and unchanged fold/partial/aux behavior. Preserve every existing association, output bit, routing and cohort decision; no automation, RT-2, RT-11 or allocation-removal work. The arena checks must remain bounded by W<=8 and execute in release.

Implement the five corrections together and attach:

- Debug and release rejection tests for one-plane arena, unsupported W, duplicate IDs, silence ID, unknown/out-of-bounds ID, reserved-but-unwritable ID, excessive frames and mismatched view width/capacity. Every rejection is nonpartial; all nonselected buffers and sentinels remain unchanged. Never construct aliased/out-of-bounds refs just to test subsequent rejection.
- Direct/staged/forced-scalar `to_bits` differential coverage at W4/W8 using existing hostile words and all frame_shapes, including sub-width and ragged sizes, IndexXor layout transformation and repeated blocks. A provider that declines must exercise staged fallback. Prove direct success never calls per-lane fallback and leaves staging sentinels untouched; count one transpose per tile per plane if using a test callback.
- A real ArenaMembers graph fixture with gather inputs different from scatter outputs/redirected IDs, compared with an independent staged reference. Preserve folded whole-lane callback ordering/count, partial inactive lanes, aux behavior, mono/collapse, PDC and existing chain/transpose/fold counters. Repeated rendering after setup must allocate/free zero using the existing allocation-probe mechanisms, not a new framework.
- Focused engine/rack/graph tests, new rejection tests in release, realtime gate plus mutation suite, rack/workspace policy, fmt/diff, and applicable clippy. Report actual counts and exact commands. No timed run for iteration.

The small matching #399 console runner/preflight registration is still an assigned delivery requirement and has not been implemented. It may be included after the semantic source tranche is coherent/checkpointed as ordinary completion of the same bounded attempt; preserve all existing consumed arms and namespaces. Freeze it and unchanged workload/validators in a root commit before non-timed preflight. No benchmark runs until Astra PASS and root's frozen preflight authorization. Full workspace, supported wasm/artifact/publisher/browser qualification and actual PR review remain later delivery gates; missing those at this candid source checkpoint is not an instruction to race them before fixing safety.

Root checkpoints this failed attempt intact and assigns Sol, not another Luna correction. Sol gets at most two implementation attempts; each receives one adversarial verdict. After attempt three fails, preserve evidence and rescope, never a hidden fourth pass. Do not weaken safety/identity tests, policy markers or descriptive-measurement rules to declare success.


Root assigns attempt 2 to Sol against this complete bounded revision. Timing remains unauthorized.
