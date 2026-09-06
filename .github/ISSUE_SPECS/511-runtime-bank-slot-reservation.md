# #511 — Reserve runtime bank-slot ownership before graph admission

#511 is the numbered prerequisite for #478; numbered/base Astra review is required before implementation. Initial source inspection: engine-rt8-plan e147263a on delivered main1543c4c2. Current numbered base: delivered main0b8cf178 plus #509 closure record fb657a01. #478 stays queued and consumes no attempt. This is one missing resource component, not certification that all graph ownership is already accounted.

## Concrete source finding

`graph/src/runtime.rs::chain_for` (1605 onward) allocates `stages` with capacity `run.len()`. Each Membership contributes at most one stage; successful adjacent builtin pairing reduces two memberships to one, factory refusal retains two. Therefore output slot count S <= run length R. Runs consume each prepared bank membership once; over the whole plan sum(R) <= prepared effect bank count + planned builtin bank count, called N below. Merging runs and successful pairing cannot increase N. `bank_chain` at1413 currently maps the stage Vec to a collected BankSlot Vec, cloning the W-byte mask once per slot. `rack::BankChain::new` retains the slot boxed slice and chain mask. These are real heap owners; the original iterator collection does not expose a source-level capacity bound suitable for the proposed reservation.

`graph-compiler/src/banks.rs::effect_bank_resource`332 charges prepared-bank array, member IDs/strings, scratch, per-member output storage and the *original* effect mask. It does not charge the runtime slot array or cloned slot masks. `builtins-compiler/src/lib.rs::builtin_bank_resource`1174 charges its prepared-bank array, members, processor payload and scratch, not runtime slots/masks. Surplus scratch from merged runs (runtime1609 comment) is not an attributable slot reservation and fails for single-slot runs.

Graph caps already cover graph_metadata_bytes, incremental_plan_bytes and largest_allocation_bytes (compile.rs610–615). `checked_add_scalar_owners` in graph/lib.rs219 demonstrates a checked, transactional owner-component addition to graph_metadata/incremental/session_plus_plan and largest. No new cap field or wire/resource schema is needed.

## Smallest product and fixed implementation route

Add one explicitly named bank-slot ownership reservation, computed before existing graph-cap admission and retained in the published estimate. Use a small Rust resource component analogous to GraphScalarOwnerResourceEstimate, not a general budget framework. Its total is deliberately a conservative *reservation*, not an exact retained-byte claim. Keep canonical semantic estimate captured before target-selected owner additions (compile.rs534) unchanged.

Calculate N from actual `banks.len()` plus the bank_count returned by the existing read-only `graph_builtin_bank_resource` preflight. Compute that builtin preflight once, keep it for the existing capped-estimate fold, and preserve effect/scalar/builtin error ordering. Fold this new component into `estimate` once, after those existing successful preflights and before caps; then clone/fold builtin payload as today. Published plan estimate must already carry the reservation; later `with_builtin_banks` must not add it a second time. Do not infer N from track count or possible effect slots. Empty/scalar populations charge zero. For supported vector banks use actual maximum prepared/planned width W (4 or8); validate/count with checked conversions, not native constants. Mixed effect/builtin runs use the combined N for the largest bound too.

Make only the following preparatory runtime change: replace bank_chain's capacity-opaque collect with an explicitly preallocated slot Vec and ordered pushes, with capacity requested for its actual stage count and no subsequent growth. Preserve stage ownership, active.clone placement/order, validation and error behavior. This is necessary allocation-shape control, not #478's activity optimization. Use the current allocator's requested layout/capacity contract; retain an assertion/proof that observed slot capacity does not exceed the reserved count. Do not paper over an unexpected capacity with a test-only assertion: if that contract cannot be established, stop and return the precise allocation API obstacle before coding a broader strategy.

Freeze a conservative inequality using target-native layouts, independently repeated in the test:

- F = size_of::<Box<dyn rack::BankStage>>(); B = size_of::<rack::BankSlot>(); W is mask bytes (bool size verified as1).
- C = N * (F + 3*B + 3*W), all arithmetic checked.
- L = max(N*F, N*B, W) for N>0; otherwise0.
- Charge C to graph_metadata_bytes, incremental_plan_bytes and session_plus_plan_bytes; largest_allocation_bytes becomes max(old,L).

The terms are named coexistence reservations: one stage-pointer Vec; old public slot Vec; a possible boxed-slice shrink destination; one future prepared-slot destination no larger than B per slot; original/chain mask ownership, cloned slot masks, and one transient per-membership mask allowance. Some cannot coexist in the current implementation; that is intentional conservative headroom, not subtraction from unrelated owners. Original effect masks may already be charged: explicitly tolerate that double reservation rather than changing their existing accounting. Across chains each term sums to at most its N-based bound. `#478` must later prove its private element <=B, no greater destination capacity, and retained/largest/conversion overlap <=this named reservation before using it. This prerequisite does not implement that conversion.

Actual retained slot component <= N*(B+2*W); current and proposed bounded conversion coexistence <=C; each component allocation <=L. These are component inequalities, not a claim the sum of every allocation made by graph bind is <=C. Stage objects, controls/observations, staging PCM, other maps/vectors and overall replacement-plan coexistence remain their existing separate contracts. Record separately any concrete additional omitted owner encountered; do not silently claim it covered by this reservation or expand this issue to it.

## Exact paths

Production: `crates/graph/src/lib.rs` (component type/checked fold and narrow public Rust calculation bridge); `crates/graph/src/runtime.rs` (actual layout calculation, explicit slot capacity, feature-gated test seam); `crates/graph-compiler/src/compile.rs` (combined count, preflight/cap/publication integration).

Tests: existing `crates/graph-compiler/src/lib.rs` resource/transactional fixtures; `crates/builtins-compiler/tests/allocation_tracker.rs` existing allocator and graph-render fixtures. Permit only a cfg(test)/existing test-support adapter in `crates/builtins-compiler/src/lib.rs` if needed to return the existing actual bound fixture and its allowance to that integration test. No rack production edit, Cargo/dependency change, CAPI ABI/resource schema, runtime scheduler/program changes or new allocator. Numbered spec/evidence are the only other initial paths. Existing downstream independent numeric resource mirrors may need a separately reviewed exact expectation amendment once the reservation delta is derived; do not blanket-repin them or reinterpret processor bytes.

## Finite discriminating evidence

1. Independent literal-layout arithmetic for N=0,1 and a mixed/multi-slot population, W4/W8. Check C/L, checked overflow, and unchanged estimate on failure. Explicitly check all four affected estimate fields and no change to bank payload/scratch/numeric graph identity. Exact-cap acceptance and cap-minus-one refusal for graph/plan/single-allocation use a fixture where this component actually determines the tested threshold, not an unrelated larger owner. Reuse `post_bank_graph_cap_rejects_transactionally_with_both_prepared_inputs` and scalar-owner cap fixture patterns; preserve returned prepared ownership and pre-existing earlier diagnostic priority.
2. Reuse the current thread-local TrackingAllocator in builtins-compiler allocation_tracker. Add test-local byte/largest counters to its existing alloc/alloc_zeroed/realloc/dealloc hooks, correctly using realloc's NEW requested size. Do not use process-global calls or aggregate allocation totals as a live peak. Prefer the conservative coexistence inequality above: measure actual requested layouts/capacities and retained releases for an isolated invocation of the same production bank_chain construction, then establish their named sum <=C. Construct/destroy unrelated stage/scratch owners outside the attribution interval, or explicitly subtract only independently identified owners; do not classify arbitrary same-sized allocations as slots. A narrow existing graph/test-support adapter may expose the construction boundary and retained slot facts, never a public production diagnostic or second constructor implementation. Include positive allocation/free/reallocation liveness controls for any added byte counters. Keep tracking allocation-free.
3. Exercise actual bound graph with unpaired multiple slots and successful builtin pairing using the existing audit_graph_render fixture, checking actual R/S and charged N. Retained typed slots/masks and largest observed component must satisfy the inequalities; after owner drop, corresponding allocations are released off render. Existing repeated render zero alloc/free remains mandatory. No new general fixture corpus or mechanism mutation campaign is needed for this accounting prerequisite.

Run the two newly named focused tests (one accounting/caps, one physical slot ownership) with nonzero selected counts in debug/release, then existing graph-compiler lib and builtins-compiler allocation_tracker suites in both profiles. Use the repository's existing test-support feature combination for allocation_tracker; record exact effective command/feature selection before the first run. Strict affected all-targets Clippy, fmt, diff, graph/realtime/workspace policies. Reuse current native/Wasm compilation during immutable delivery; no timing or benchmark runner. Actual shipped artifact mismatch, if any, requires the normal separate observed-pin/current-consumer ruling; no automatic numerical repins.

## Delivery and remaining boundary

This issue closes when actual compiler admission reserves this named slot component, independent physical bounds and transactional cap behavior pass, and ordinary delivery/required CI succeeds. #478 can then receive a fresh base review and implement only its already-frozen packed-mask outcome under these inequalities. Neither issue may claim all global resource owners audited. This is a half-day-sized admission/component correction plus two existing fixture extensions; a newly discovered allocator-capacity or unrelated-owner problem is a precise stop/successor decision, not permission for a general resource rewrite.

Fresh Luna1 followed by one consolidated Astra verdict; Sol2 and Sol3 only if needed, hard stop/rescope after three failed attempts. Parent #478 accounting stop remains binding until this prerequisite is delivered. No implementation, tests/builds, timing, repository/Git/GitHub mutations were performed for this brief.

## Numbered Astra scope approval and frozen gate clarification

# #511 numbered scope/base review — PASS

Reviewed clean 7f93a324eea5c8cddae8eb06692a8f9de93b5965 in engine-slot-reservation. GitHub511 is OPEN with matching number/title and exact local body. The body from “Concrete source finding” onward equals the accepted accounting prerequisite brief verbatim. Versus delivered0b8cf178 the delta is only the numbered spec and509 closure record. Graph, graph-compiler, builtins-compiler, rack, Cargo and .cargo inputs are unchanged from inspected e147263a. No base drift changes the count, capacity or accounting premises.

Approve fresh Luna1 for this prerequisite only. Preserve combined membership-count reservation, checked transactional folding, canonical semantic estimate separation, explicit allocation-shape bound and independent physical inequalities. #478 remains queued until prerequisite delivery and fresh base review. This PASS does not establish an actual capacity/peak measurement or certify unrelated owners.

Freeze these two new focused test identities before implementation:

1. `tests::runtime_bank_slot_reservation_is_published_and_capped_transactionally` in crates/graph-compiler/src/lib.rs. This single fixture covers the specified arithmetic/empty/overflow and independently discriminating publication/cap/returned-ownership cases; existing relevant fixtures remain.
2. `actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation` in crates/builtins-compiler/tests/allocation_tracker.rs. This covers the finite actual construction/retention/release inequalities and bound-graph membership/pairing proof using the existing allocator. Do not accept layouts merely because they share an allocation size with a slot owner.

Exact focused commands (each selects one test) are:

```
cargo test --locked -p graph-compiler --lib tests::runtime_bank_slot_reservation_is_published_and_capped_transactionally -- --exact
cargo test --locked --release -p graph-compiler --lib tests::runtime_bank_slot_reservation_is_published_and_capped_transactionally -- --exact
cargo test --locked -p builtins-compiler --features test-support,graph/test-support --test allocation_tracker actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation -- --exact
cargo test --locked --release -p builtins-compiler --features test-support,graph/test-support --test allocation_tracker actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation -- --exact
```

`allocation_tracker.rs` is crate-gated on builtins-compiler/test-support; the existing prior owner evidence used that feature. Graph already declares its own test-support feature. Enable that existing dependency feature explicitly for the approved graph construction-boundary adapter; no Cargo declaration or normal feature change is necessary. Graph-compiler already enables builtins-compiler/test-support as a dev dependency. The named graph compiler test lives in its existing `tests` module.

Full affected commands are `cargo test --locked -p graph-compiler --lib` and `cargo test --locked -p builtins-compiler --features test-support,graph/test-support --test allocation_tracker`, each repeated with `--release`. Record actual counts, individual numeric statuses and source identity. Strict affected all-targets Clippy must cover normal and enabled test-support compilation; retain fmt/diff and existing graph/realtime/workspace policy gates. No expanded corpus, benchmark, new allocator, public resource schema or automatic pin changes follow from naming these gates.

Root should append these exact names/feature commands to the synchronized numbered decision record before assignment. Existing capacity-contract and unrelated-owner stops remain intact: an unsupported allocation-capacity premise is a precise returned obstacle, not permission to substitute a test-only assumption. No implementation, builds, tests, timing or repository/GitHub mutations performed in this review.

## Gate feasibility clarification adopted with this PASS

Do not grow a session until slot storage happens to dominate every processor/scratch allocation. For the new component's L, independently test the checked fold with a small synthetic prior estimate whose largest field is below L, then with an existing larger value: require max(old,L) exactly, plus zero/overflow transactional behavior. In the small real compiler fixture, test admission at its independently computed *whole-plan* largest and refusal one byte below that whole-plan maximum. Attribute that rejection honestly to the whole-plan maximum; it need not be a slot allocation. Graph/plan total boundaries must include the exact new reservation delta. Together these discriminate the component calculation, integration and cap consumption without a huge graph. This clarifies the earlier brief's “component actually determines threshold” requirement: that discrimination belongs in the direct checked-fold fixture when real owners dominate.

Keep the physical adapter construction-specific: a feature-gated prepared-input/owned-chain test bridge to the SAME bank_chain function is sufficient. Prepare fake stage/scratch owners outside observation, invoke the actual ordered slot conversion inside the existing armed interval, expose only typed slot/mask counts/capacities needed for the fixed inequalities, and release owners on the control thread. Account explicitly for incoming stage Vec and mask allocations when their frees occur inside an observed interval; do not subtract untracked bytes from zero or misidentify stage/scratch frees as slot releases. Use a second, existing actual bound-graph fixture for membership/pairing reachability. No generic callback observer registry, allocation-ID database or second constructor is needed.

The existing allocator realloc branch reports the OLD layout to its phase-two recorder. This is not evidence of the new allocation's byte size. Add the narrow test-local requested-byte observation using `Layout::from_size_align(new_size, old.align())` (checked outside any fallible assumption as appropriate to GlobalAlloc's contract) for the NEW request. The original phase-two record semantics remain unchanged in this issue. A positive controlled realloc must prove the new counter sees the increased size; do not silently change old recorder behavior and call its prior evidence unchanged. For a conservative transient bound a realloc may coexist with its old allocation; do not interpret net growth as its maximum live request.

Root activates fresh Luna attempt 1 at this documentation checkpoint. Implementation is limited to the approved reservation prerequisite; #478 remains queued.

## Luna attempt 1 focused checkpoint

Both frozen exact tests pass in debug and release (one selected each). The first coherent source tranche is checkpointed before full affected suites and consolidated review. Initial failures and corrected outputs remain under /tmp/511-luna1-* for packaging; root captured current source SHA256 independently. Execution records initially omit source identities and some metadata contains literal escaped newlines; do not treat later identity supplements as contemporaneous captures.

Implementation adds maximum_mask_bytes to the existing Rust builtin-bank resource component and fills it in builtin_bank_resource to obtain actual planned width. This extends the initial builtins-compiler production path allowance and requires an explicit Astra scope decision before acceptance; checkpointing is not approval. No wire schema, cap field, Cargo or rack production change is made. Remaining full-suite, ownership/pairing gate completeness and adversarial review are pending.

## Narrow width-metadata scope decision

# #511 width metadata scope amendment — APPROVED NARROWLY

Reviewed clean source checkpoint `f8215094d5f6e4cf74466a3111aaeccf29858c10` in `/home/bl/misofm/engine-slot-reservation`, the numbered issue spec, its prior Astra scope review, and the applicable AGENTS.md. This is a scope decision only, not a consolidated attempt verdict or acceptance of the physical evidence.

Approve extending the production allowance in `crates/builtins-compiler/src/lib.rs` solely to populate `GraphBuiltinBankResourceEstimate::maximum_mask_bytes` in `builtin_bank_resource` and combine it by maximum in `add_bank_resource`. The corresponding one-field addition to the existing graph Rust resource component and its compiler consumption are within this amendment. Root should record this exact exception in the numbered spec and synchronize it through the normal checkpoint workflow.

The extension is proportionate: the existing read-only preflight already knows both actual planned groups and their selected width. Returning one scalar with its existing result avoids a second plan traversal or an additional preparation API. Its producer uses the same width that the actual lowering passes to the bank constructors and `AoSoaScratch::new`. Aggregation by maximum is correct for mask width; it must not be summed or interpreted as an independently retained payload charge.

A smaller alternative exists and is justified by current source: when the preflight's builtin bank count is positive, `BankWidth::for_backend(rack_cohorts.dispatch)` gives its actual planned width, because `graph_builtin_bank_resource`, `planned_builtin_bank_members`, and `into_graph_artifact_with_banks` all derive width from that same dispatch. Combined with the maximum of the actual prepared effect masks, this would satisfy the frozen requirement without the new field. It would not be an unjustified native/backend constant shortcut. Nevertheless, the submitted three-line producer/aggregate extension keeps this fact with its existing owner and is narrow enough to approve; no churn to the alternative is required for scope compliance.

The approval preserves the existing wire/resource-cap schema, canonical semantic estimate, checked accounting, diagnostic priority, preflight/attach ownership, and deferred builtin payload fold. It permits no new runtime diagnostic, dependency, rack edit, alternate constructor, global resource framework, or automatic downstream repin. The maximum field is descriptive input to the separately named slot reservation, not a new cap or a second charge at attachment.

One detail remains for the consolidated implementation review: `builtin_bank_resource` currently reports a width even for an empty `groups` slice, and `planned_strip_banks` returns all three stage entries even when their groups are empty. Consequently this field is not literally an observed maximum over nonempty planned banks in that case. The N=0 component presently returns zero, so this observation alone does not establish an admission bug. The final review should ensure absent populations cannot widen another population's reservation; a zero-for-empty field assignment, if needed, is within this same narrow producer allowance. This is recorded as an implementation observation, not a partial attempt verdict or authorization for a separate revision round.

No builds, tests, timing, repository edits, Git mutations, or GitHub mutations were performed. Only this requested temporary decision report was written.

## First attempt final validation disposition

Full graph-compiler suites each report60 passed/4 failed at existing resource mirrors; allocator suites each report7 passed. Initial strict Clippy found6 inconsistent test literal groupings; the mechanical underscore-only correction passes strict graph-compiler Clippy and is adopted as a separate recoverable checkpoint. Existing numerical mirrors were not repinned. Actual-bound unpaired membership/slot inequality proof remains incomplete; remaining static gates were not executed after the correction stop. Root requests one consolidated adversarial verdict on this candid attempt, rather than claiming focused success closes the issue.

## Consolidated Luna attempt 1 FAIL and Sol attempt 2 brief

# #511 consolidated Luna attempt 1 — FAIL

Reviewed clean pushed checkpoint `cdae5a57` in `/home/bl/misofm/engine-slot-reservation`, source checkpoint `f8215094d5f6e4cf74466a3111aaeccf29858c10`, the six test-literal separator corrections in `7e61a818`, the frozen numbered spec and adopted width amendment, relevant graph/builtin/rack construction paths, and the preserved attempt package. This is the single consolidated attempt-1 verdict. Proceed to one bounded Sol attempt 2; #478 remains queued. Do not treat the earlier width scope approval as an implementation PASS.

## What is established

The production component implements the frozen C/L formulas with checked arithmetic and a clone-before-mutation estimate fold. Its combined count comes from the checked count of actual prepared effect banks plus the existing builtin preflight count. Effect masks are validated by the preceding effect-resource calculation; builtin width metadata originates in the existing planner/preparer. The compiler publishes a slot reservation and includes it in its cap candidate. The separate published and capped estimate folds do not themselves double-charge an attached graph. Canonical semantic estimate capture remains before target-specific owners. The runtime change preserves ordered stage moves and mask cloning while requesting slot capacity explicitly. No rack production, Cargo, wire cap, or scheduler change was introduced.

The new physical fixture reaches the actual production `bank_chain`; its three identity stages are zero-sized, its three-active-of-eight mask avoids full-bank staging, and its prepared scratch/chain mask/stage vector precede the observed conversion. That is a useful narrow construction seam. Successful builtin pairing and the existing repeated-render allocation gates also execute. These facts do not establish the missing quantitative assertions below.

## Required corrections in the same bounded slice

### 1. The physical evidence does not prove retained release, largest allocation, or conversion coexistence

`crates/builtins-compiler/tests/allocation_tracker.rs:54–81, 137–157, 500–585`:

- Incoming stage-vector bytes are not in the starting live balance, yet their free is subtracted. Saturating subtraction hides that attribution error. Immediately before final destruction the test resets live bytes to zero again; every destruction can therefore leave zero regardless of leaked slot owners. `LIVE_FREES` is not reset for that release window and `>0` is not an attributable slot-release assertion.
- The allocation/free evidence uses `.any()` on size alone and checks only nonzero counts. It does not account for the exact constructor events or distinguish the scratch and original mask frees during destruction. The helper's typed sizes are useful, but do not turn arbitrary size matches into owner identities.
- `LARGEST_REQUESTED_BYTES > 0` is only liveness. Comparing L against one slot element (`slot_bytes / 3`) does not bound the actual whole slot-array request. Neither the observed largest component nor each concrete component request is compared with L.
- The test separately checks that a stage-vector request and slot-array request fit C. It never checks their named coexistence sum against C. The retained inequality is restated from the helper's expected sizes, with no complete attributable release corroboration. `PEAK_LIVE_BYTES` is not consumed. Its realloc implementation subtracts the old request before adding the new one, so it cannot establish old-plus-new coexistence anyway.
- `retained_slot_capacity` is assigned the incoming slot count before `BankChain::new`; it is an inferred boxed-slice length, not an independently observed retained capacity. Name/document this inference honestly and corroborate retained storage through the construction/release evidence.

Minimal correction: retain this existing narrow seam and allocator. For the isolated ragged fixture, enumerate the constructor's complete request/free shape, use fixed typed facts to account for pre-existing owners, and assert exact event counts/layouts rather than an arbitrary matching layout. Because this fixture has zero-sized stage payloads and no tiled staging, its conversion requests can be attributed without a pointer registry: one explicitly sized slot Vec plus S cloned masks, and the incoming stage Vec release. During destruction account explicitly for the original mask and the two pre-existing scratch planes; they are not newly observed slot allocations. Seed the appropriate opening balance or keep explicit bounded allocated/freed byte sums with an underflow/overflow failure flag; never silently subtract untracked ownership from zero. Reset release-window counters and assert the specific complete releases while preserving owner lifetime through observation.

Independently calculate F, B, W, retained slot/mask ownership, largest component request, and the named conservative coexistence sum in the test. Assert retained <= N*(B+2W), each actual component request <= L, and the combined conversion bound <= C. Include the incoming stage capacity, which may exceed S after pairing but is bounded by R. A conservative named sum is enough: no generic live-heap framework or exact process-wide peak is required. Keep the positive new-size realloc control and add discriminating allocation/free checks for the counters actually used. If a peak counter is retained as evidence, its semantics must include old/new realloc coexistence; otherwise remove that unused claimed evidence. Preserve the old phase-two recorder's existing semantics.

### 2. Actual bound unpaired/multiple-slot membership and charged N are unproved

The final graph call is only `test_only_prepared_pair_graph(false)` and the sole assertion is `fused_calls > 0`. There is no actual R/S witness, no unpaired multiple-slot case, and no comparison with charged N. Moreover this existing fixture constructs a test graph with an initially zero estimate and calls builtin attachment directly; it bypasses `GraphCompiler`, so its presence alone cannot prove compiler reservation publication.

Minimal correction: extend the existing test-support bridge with fixed, allocation-free construction facts at `RuntimeParts::chain_for`/`bank_chain` (aggregate counts/maxima or a bounded fixture-specific record, not a callback registry). Return the existing prepared fixture's actual count/allowance through its permitted adapter. Exercise both normal pairing and its existing non-pairing control-delivery variant; the fixture already takes `between_render_calls`, and its public allocator adapter currently hard-codes true. An observation barrier alone may split the run into single slots, so require the unpaired case to demonstrate S>1 explicitly. Show S<=R, aggregate R<=N, and S<R on a successfully paired run. Relate the same count/allowance to the separately tested real compiler admission; do not describe the direct-attachment fixture as having passed compiler caps. Keep repeated actual render zero alloc/free assertions. No second constructor or new fixture corpus is necessary.

### 3. The independent accounting/cap oracle is incomplete

`crates/graph-compiler/src/lib.rs:2365–2585` has a valid literal component arithmetic comparison for selected nonzero N/W cases, a zero component case, and one early-field overflow. But it omits the explicitly approved synthetic checked-fold test where the new L controls the largest field, then where an existing larger value must remain unchanged. The real expected publication invokes the same production `checked_add_bank_slot_owners` as the implementation. A broken/no-op fold could therefore agree with its own oracle; caps derived from the resulting artifact would also accept their own erroneous boundary.

The real fixture has only builtin banks and no prepared effect entries. N=3 in the arithmetic loop is not evidence that actual effect plus builtin counts were combined. The cap loop does prove acceptance/refusal around the returned whole-plan maximum; it does not independently derive that maximum or reservation delta. Returning eight tracks and eight tails is limited evidence, not preservation of live prepared effect processors in a mixed graph.

Minimal correction: extend the same frozen test. Construct independent expected totals by direct field arithmetic and assert all four changed fields, leaving unrelated fields exactly unchanged. Add the small direct fold cases with old largest below/above literal L, zero addition, and overflow in each of the three additive fields with whole-estimate rollback. Verify bool size, N=0/scalar zero, and representative W4/W8 cases. Reuse an existing small prepared-effect fixture with builtins to demonstrate both counts nonzero and a combined N; independently compare report and attached graph estimates, unchanged bank payload/scratch and canonical semantics, and exact graph/plan deltas. Calculate the whole-plan largest from the independently derived previous owners and literal L; retain its exact-cap/minus-one checks without pretending a larger processor allocation is a slot. On rejection verify the returned live effect entries and sealed builtins, and reuse/retain existing earlier-diagnostic coverage. No large fixture or mutation campaign is requested.

### 4. Preserve the earlier builtin arithmetic diagnostic and empty-population width semantics

`compile.rs:595–647` currently computes the new combined count/mask/component before the existing `checked_add_builtin_banks` fold. If both new arithmetic and the earlier builtin fold would fail, the new `$.graph.bank_slots` diagnostic now wins. That violates the frozen preservation of existing effect/scalar/builtin arithmetic priority. Move the already-existing capped-estimate builtin fold immediately after its successful preflight, before fallible slot arithmetic, then perform the slot folds/caps. This is a local reordering, not a new admission helper framework.

`builtin_bank_resource` reports nonzero `maximum_mask_bytes` for empty groups; `planned_strip_banks` visits all three entries even when empty. Set this descriptive maximum to zero when that kind has no banks, retain maximum aggregation, and cover the empty preflight in the frozen accounting fixture. Current N=0 already yields zero reservation, and current effect/builtin widths derive from the same dispatch, so no reachable under-reservation is established from this detail. The change enforces the approved actual-population meaning and prevents an absent population from being used as width evidence. It is within the adopted narrow width amendment.

### 5. Complete the existing required gates after the coherent correction

Both full graph suites are red and the final builtin-compiler strict Clippy, fmt verification and graph/realtime/workspace policy checks are incomplete. The mechanical digit correction is already accepted and has passing strict graph-compiler Clippy; it requires no further design change. Run the frozen two exact tests and affected full suites in debug/release with the specified existing features, then the proportional static/target gates already required by the issue. Capture source identity before execution and numeric exits/counts honestly. Do not expand target matrices or run timing. Native/Wasm immutable-delivery evidence remains required through the normal delivery route; no such final delivery PASS is established by this package.

## Bounded numeric expectation amendment — APPROVED

Only the four named existing fixture bodies in `crates/graph-compiler/src/lib.rs` need the derived slot component added to their prior scalar-to-banked total equations:

| Fixture | Current first observed actual / expected | Authorized equation adjustment |
| --- | --- | --- |
| true-peak limiter, around 7476–7538 | 55,735 / 55,599 (graph metadata) | Add independent C to graph metadata, incremental plan and session-plus-plan totals |
| multiband compressor, around 7880–7933 | 333,327 / 333,191 (incremental plan) | Add independent C to incremental plan and session-plus-plan totals |
| soft clip, around 8259–8312 | 183,839 / 183,703 (incremental plan) | Add independent C to incremental plan and session-plus-plan totals |
| transient shaper, around 8642–8692 | 176,655 / 176,519 (incremental plan) | Add independent C to the existing total bank-overhead equation |

Each has ten tracks with one eligible effect per track, no builtin banks, and a scalar-only delegate comparison. Its already independently checked bank count is N=floor(10/width), or zero for scalar. Derive `C=N*(size_of::<Box<dyn BankStage>>() + 3*size_of::<BankSlot>() + 3*width_lanes*size_of::<bool>())` directly in each fixture (or a small test-local literal helper). On this native run F=16, B=32, W=8, N=1, giving C=136, exactly each observed discrepancy. On four-lane 64-bit targets N=2 gives C=248; use target-native layouts, not either hard-coded delta. These expectations must not call the production reservation/fold helper as their oracle.

Do not add C to `effect_bank_metadata_bytes`, processor payload, scratch, audio-buffer samples, latency, tail, canonical bytes or unrelated expectations. The source equations themselves explain why the later incremental/session assertions also require the delta even though each test stopped at its first failure. This authorizes those exact derived equations, not a blanket repin or any shipped artifact/hash change. Rerun the full suites to exercise the previously unreachable later assertions.

## Evidence integrity and limits

Independently verified all 56 manifest payloads against recorded SHA256 and byte lengths: no missing/mismatched entries; all 56 plus `manifest.json` are tracked (57 files). All six hashes in the final identity file match Git blobs at f8215094. The current graph-compiler test blob matches the correction metadata; Git confirms its only source difference is the six literal separator changes. The other five current source hashes match the final-suite identity.

Raw results confirm each frozen exact test selected one passing test in debug/release; graph full suites each report 60 passed/4 failed with exit 101; physical suites each report 7 passed with exit 0. Strict graph Clippy reports success; strict graph-compiler Clippy initially failed six literal-format lints and its explicitly identified correction reports exit 0. These are valid records of execution, not proof of assertions that the tests omit.

The early focused metadata does not include contemporaneous source hashes, some records contain literal escaped newlines, and several final metadata files repeat the same exit field. Preserve the original bytes and the honest README caveat. The later root identity supplement cannot retroactively establish an early invocation's immutable source identity. Final suite metadata relies on the package's separate final identity file and report for source association. No new historical certainty is claimed. The final report's obsolete description of the correction as dirty is resolved by the enclosing README/commits, not by rewriting raw evidence.

No builds, tests, timing, repository edits, Git mutations or GitHub mutations were performed during this review. Only this requested temporary review report was written. The named source/accounting/evidence corrections fit one Sol2 pass in the already permitted paths; no gate weakening, allocator database, public schema expansion, or unrelated owner audit is authorized.

Root integrated delivered maina6a59030 and #512 closure/post-main records before Sol2. The only merge conflict was #509 documentation; the completed post-main success paragraph was retained. All six #511 source files and first-attempt evidence remain identical to the reviewed cdae5a57 source. Root activates fresh Sol attempt2 for precisely this consolidated correction, with the four derived expectation amendments authorized above. No benchmark or automatic artifact pin update is authorized.

## Sol attempt 2 focused checkpoint

Sol corrected the six approved source paths and both frozen exact debug tests pass one test each. Root verified contemporaneous six-file SHA256 identities in both successful records before committing. Initial feature/compile/canonical-oracle failures remain preserved. Release/full/static gates and consolidated Astra acceptance are still pending; this checkpoint is recovery progress, not issue closure.

## Sol attempt 2 final local evidence

Source777d168f and the mechanical test-only follow-upca8f34d5 pass both frozen exact tests, full graph64/64 and allocation7/7 suites in debug/release, scoped strict Clippy and fmt/diff/graph/realtime/workspace policies. Actual compiled construction facts show pairedN6/R6/S5 and unpairedN6/R6/S6 with max per-chainS3. Raw evidence, source identities and candid operational failures are preserved in artifacts/issue511-sol-attempt2. Consolidated Astra review and immutable delivery are pending.

## Consolidated Sol attempt 2 FAIL; final Sol attempt 3 scope

Astra reviewed clean cd372f3d and accepted the corrected component arithmetic, independent mixed admission/rollback/caps, prior diagnostic ordering, actual paired/unpaired counts, attributable allocation/release/largest evidence and four derived numeric equations. All124 Sol2 payload hashes and125 tracked files verify. The remaining blocking finding is a real existing consumer regression: scripts/preflight-builtins-benchmark.sh invokes builtins-compiler tests with only its test-support feature, but the new physical test also needs graph/test-support. Passing explicit workspace configurations does not preserve this isolated consumer. The earlier report's operational-only classification is rejected; all raw failures remain preserved. Full consolidated review: /tmp/astra-511-attempt2-review.md, to be retained with final-attempt evidence.

Astra approves exactly two final source corrections:

- In crates/builtins-compiler/Cargo.toml, change the existing test-support feature from [] to ["graph/test-support"]. This is forwarding over the existing dependency, with no new/default dependency or normal production instrumentation. It is the sole exception to the earlier no-Cargo-edit freeze.
- In the existing allocation_tracker test, compute conversion_coexistence = n * f + n * b + (n + 1) * w. The complete observed stage/slot/mask coexistence includes the original8-byte chain mask:176<=408. The earlier168 subset remains an accurately preserved partial observation, not the complete component bound. Correct the new report/decision wording accordingly; no new test framework or allocator.

Do not edit the historical preflight, seals, digests, test filter or physical-test enablement. Root activates final Sol attempt3 for these exact corrections. Run the preflight's isolated Cargo command directly (one actual selected test), never the full sealed/timed preflight:

```
cargo test --locked -p builtins-compiler --features test-support phase_two_allocator_layouts_match_the_checked_resource_report
cargo clippy --locked -p graph -p graph-compiler -p builtins-compiler --all-targets -- -D warnings
```

Run the frozen focused and affected debug/release suites and proportional static checks on final identified source, including the manifest in source identity. This is attempt3, the final allowed implementation attempt; one consolidated Astra verdict follows. If it fails, stop and rescope once rather than silently retry a fourth time. Immutable native/Wasm/current-artifact qualification follows only after acceptance. No benchmark/timing, new fixture corpus, broad manifest/dependency changes or preemptive pin updates are authorized.

## Final Sol attempt 3 focused checkpoint

Exactly the two approved lines changed: existing test-support forwards graph/test-support, and complete coexistence includes the original mask. The old isolated preflight Cargo command and frozen physical debug test each pass one test; root verified contemporaneous seven-file hashes before checkpoint. No full preflight/timing was run. Final combined Clippy, remaining frozen suites/static gates and Astra verdict remain pending.

## Final Sol attempt 3 local evidence

Source35dc8d4d passes all18 captured statuses: the exact existing isolated consumer selects1 passing test, previously failing combined Clippy passes, frozenexacttests1 each, fullgraph64 each and allocation7 each in debug/release, plusstaticpolicies. Complete coexistence is176<=408. Artifacts/issue511-sol-attempt3 preserves raw seven-file source identities/statuses/report and the full prior Sol2FAIL ruling. No native/Wasm/current-artifact delivery is claimed yet; final consolidated Astra verdict pending.

## Final consolidated Astra Sol attempt 3 PASS

# #511 consolidated final Sol attempt 3 — PASS (source)

Reviewed clean `8d07e25611b844b98e8b0acdbc185c11f9ef72f3` in `/home/bl/misofm/engine-slot-reservation`, final source checkpoint `35dc8d4dc90c352bc1b328ec57852fa520a1e197`, the complete numbered issue and adopted amendments, applicable AGENTS.md, the prior consolidated Sol2 review, the final diff, relevant unchanged accounting/construction/test paths, and all three preserved evidence packages. Local HEAD and the existing origin tracking ref both identify `8d07e256`; no remote fetch or GitHub-state verification was performed by this reviewer.

This is the single consolidated final-attempt verdict. The accepted Sol2 core proof composes with the exact final compatibility and complete-mask corrections. No blocking source finding remains for the bounded #511 contract. Root may proceed to the already required immutable workspace/native/Wasm/current-artifact qualification and delivery workflow. This source PASS does not itself close #511 or release #478 from its prerequisite-delivery and fresh-base-review conditions.

## Final correction and compatibility

Relative to reviewed `cd372f3d`, the source diff is exactly one removed/added line in each of two files:

- `crates/builtins-compiler/Cargo.toml` forwards the existing `test-support` feature to `graph/test-support` over the already existing graph dependency.
- `crates/builtins-compiler/tests/allocation_tracker.rs` includes the original mask in `conversion_coexistence = n * f + n * b + (n + 1) * w`.

Everything else since that review is the authorized numbered decision/evidence record. `Cargo.lock`, other manifests, default features, dependency declarations, the historical preflight and its test filter, production instrumentation enablement, seals and artifact pins are unchanged. The graph seam remains explicitly feature-gated; normal production dependencies gain no default instrumentation. The final source checkpoint introduces no extra constructor, allocator, runtime mechanism or numerical expectation revision.

The old consumer in `scripts/preflight-builtins-benchmark.sh` still invokes `cargo test --locked -p builtins-compiler --features test-support phase_two_allocator_layouts_match_the_checked_resource_report`. The preserved immutable rerun uses that exact command, returns status 0 and actually executes its named allocation test: 1 passed, 0 failed. Other test binaries select zero tests, which does not replace that positive selection. The previously failing combined `cargo clippy --locked -p graph -p graph-compiler -p builtins-compiler --all-targets -- -D warnings` also returns 0. Together the source feature edge and these direct results resolve the concrete Sol2 compatibility blocker without changing its historical classification.

## Complete contract acceptance

The previously accepted production implementation and core fixtures are unchanged by this final correction. Their acceptance remains part of this consolidated verdict:

- Checked target-layout arithmetic implements C=N*(F+3B+3W) and L=max(NF,NB,W), with zero for an empty population. The estimate fold uses a temporary copy, changes the three additive totals and largest maximum, and commits only after every addition succeeds.
- Compiler N combines actual prepared effect banks and the single planned builtin preflight. Width comes from actual prepared/planned masks; empty builtin populations report zero. Existing effect/scalar/builtin arithmetic diagnostic ordering is preserved before new slot arithmetic. The published and capped estimate copies each receive the reservation once, and later builtin attachment does not repeat it. Canonical semantic capture stays before target-selected additions.
- Ordered construction requests the slot vector for its actual stage count and does not grow it. Stages and masks preserve their established ownership and order. Actual membership accounting proves S<=R<=N, including successful pairing and unpaired multiple-slot reachability. The accepted compiled facts remain paired N6/R6/S5/maxR3/maxS2 and unpaired N6/R6/S6/maxR3/maxS3; final repeated-render tests exercise both paths with the existing zero-allocation/free audit.
- Independent literal arithmetic, all-field fold equality, below/above largest cases, zero addition and rollback for each additive-field overflow remain covered. The real mixed compiler fixture has live effect and builtin populations, independently derives their combined reservation and attached estimate, preserves unrelated payload/scratch fields and canonical bytes, and exercises exact/minus-one graph, plan and whole-plan-largest caps with returned prepared ownership. Whole-plan-largest rejection is not misattributed to slot dominance.
- The four prior numeric fixture amendments remain only their approved target-native slot-reservation additions to total equations. No effect payload, PCM, latency, tail, canonical identity or shipped artifact expectation is repinned.

The physical fixture still reaches the same production constructor and exhaustively identifies its isolated requested/free layouts. For N=S=3, F=16, B=32 and W=8, construction requests one 96-byte slot array and three 8-byte mask clones, and frees the incoming 48-byte stage vector. L=96 bounds each named request. Complete stage/slot/mask coexistence is now explicitly 48+96+24+8=176<=C=408. The original 168-byte subset remains correctly preserved as a partial observation. Retained slot/chain-mask storage is 128<=144. Destruction observes one slot array, four masks and two separately identified 32-byte scratch planes: seven frees, zero allocations and a checked zero closing balance. This is an attributable component proof, not global bind-heap or replacement-plan accounting. Future #478 still owes its own element-size, destination-capacity and coexistence proof under the reservation.

## Final evidence integrity and gates

Independently verified every manifest payload SHA256 and byte length: Luna1 56 payloads/57 tracked files, Sol2 124/125, Sol3 75/76. No missing, mismatched or untracked package entries were found. The first two packages have no diff since `cd372f3d`. The Sol3 archived prior review is byte-identical to `/tmp/astra-511-attempt2-review.md`.

All 18 final-attempt statuses are numeric 0. Each associated metadata file contains seven source hashes matching the current files. The 16 clean-source records identify `35dc8d4d`, include effective PATH and empty porcelain markers, and their seven hashes also match those exact Git blobs. The two earlier dirty-source runs are identified as such; the first old-consumer record's omitted environment is candidly retained and superseded for qualification by the complete clean-source rerun, not reconstructed.

Raw final results establish:

- Both frozen exact tests: 1 passed each in debug and release.
- Full graph-compiler lib: 64 passed in each profile.
- Full allocation tracker: 7 passed in each profile.
- Existing isolated consumer: its intended test passes once.
- Combined strict all-targets Clippy and explicit builtin+graph test-support strict Clippy: status 0.
- Format, diff and graph/realtime/workspace policies: status 0.

Historical failures remain preserved. Source acceptance relies on the identified successful reruns and unchanged accepted core evidence, without upgrading the earlier provenance limitations or treating status alone as proof of an omitted assertion.

Immutable workspace/native/Wasm checks, ordinary current-artifact qualification, required CI, final GitHub synchronization and closure remain root's delivery obligations. No full historical preflight, benchmark, timing, automatic pin update or expanded qualification project follows from this PASS. No builds, tests, timing, source edits, Git mutations or GitHub mutations were performed during this review; only this requested temporary review report was written.


## Bounded delivery integration rulings

The immutable workspace run on 0f30be0a stopped at the C API numeric mirror (status101); the ordinary builder compiled successfully then rejected the old artifact pin (status1). Both original raw captures are preserved. These are delivery integration corrections following final source PASS, not another production implementation attempt.

# Astra #511 artifact integration ruling — approved, bounded

Reviewed clean frozen `0f30be0a76f89994059e9ef435b8ddb6edcc152f` in `/home/bl/misofm/engine-slot-reservation`, the accepted final source review, `/tmp/511-delivery-build.py`, actual builder command/log/status, ordinary builder source and existing #496 integration route. The only change since accepted `8d07e256` is the #511 PASS adoption in its issue spec; all seven accepted source files and the build inputs remain identical.

The builder metadata identifies this exact frozen source, output `/tmp/engine-511-qualified` and outer `CARGO_TARGET_DIR=/tmp/engine-511-artifact-target`. The wrapper checks the accepted HEAD and clean worktree before invocation and checks unchanged HEAD/cleanliness afterward. The ordinary script internally isolates its Wasm compilation in its own temporary target directory and removes it on exit. Its log records successful release compilation, then numeric builder status 1 at the actual artifact comparison:

- Existing expected SHA256: `25c1b72a65ebfd081c74d431614cfba42492e95e490cc4d7c203ee14fe8737e9`.
- Observed SHA256: `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`.

The expected value matches the current pin file. The output directory is empty: the mismatch occurs before publication, and the temporary module has been cleaned up. This review verifies the source/command lineage and authentic observed mismatch; it does not pretend to independently hash a retained or published module that is absent.

Approve exactly the observed SHA256 replacement in `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, the corresponding decision/evidence checkpoint, and the existing seven-step consumer pipeline: ordinary verified rebuild, static/object/ABI checks, resource gate including its 26 rejection controls, hermetic worklet, npm installation, current three-browser qualification with existing self-tests, and generated matrix check. Verify the newly published module's actual bytes/SHA against this exact pin and all current consumer records. Candidate/hash identity records may change as required by that existing route.

This approval does not authorize numerical resource or PCM expectation repins, canonical changes, corpus pins, schemas, gate/script/CI/lint changes, or further runtime implementation. Any actual numerical or additional source discrepancy requires a separate concrete ruling; do not adjust expected numerical outputs to obtain a passing gate. The anticipated artifact identity change does not establish any unobserved numerical outcome.

The immutable workspace/supported-target/native ABI sequence remains pending in the root's active session. Do not mutate its tracked worktree, including pin/spec changes, until that sequence is terminal. This ruling supplies no implied permission to interrupt or contaminate that immutable run and makes no claim that it has passed. Preserve the original mismatch records unchanged.

After all mandatory terminal qualification, the existing final actual-PR exact-head review, required CI and GitHub delivery synchronization remain necessary. #478 remains subject to completed #511 delivery and fresh base review. No timing, benchmark, new matrix or extra fixture framework is authorized. Read-only review; no builds/tests, source/spec/Git/GitHub mutations or timing were performed. Only this requested temporary ruling was written.

# Astra #511 C API numerical integration ruling — approved, bounded

Reviewed clean frozen `0f30be0a76f89994059e9ef435b8ddb6edcc152f` in `/home/bl/misofm/engine-slot-reservation`, the accepted #511 source contract, authentic `/tmp/511-immutable-workspace.{command.json,log,status}`, `crates/capi/tests/resource_lifecycle.rs`, the production replacement-cap addition and graph preparation/report paths. The workspace run is terminal with status 101; its first failing integration binary reports 3 passed/1 failed. The ordinary artifact builder is also terminal with its separately ruled status-1 hash mismatch. No pending immutable-run permission is inferred from elapsed time.

The failure is a stale independent test mirror of the newly accepted reservation, not evidence of another production defect. Approve the exact test-only arithmetic integration below in `crates/capi/tests/resource_lifecycle.rs`, with corresponding issue/evidence documentation. This does not reopen #511's three production implementation attempts or authorize a disguised fourth revision.

## Independent derivation

The existing scratch fixture is nine homogeneous soft-clip tracks on its frozen eight-lane native model. It prepares one full effect bank (floor(9/8)=1; the ninth effect is scalar) and two banks for each of the three builtin strip stages (3*ceil(9/8)=6). Therefore the charged membership count is N=1+6=7, irrespective of later adjacent builtin pairing.

The stage owner is a boxed trait object, independently represented by its data/vtable pointer pair: F=16 on this native target. A current BankSlot consists of that stage owner and a boxed bool slice (data pointer plus length), yielding B=32. Its eight-lane bool mask has W=8 with bool size 1. Thus:

- C=7*(16+3*32+3*8)=952 bytes per plan.
- L=max(7*16,7*32,8)=224 bytes for the named largest component request.
- Retained and conservative coexistence reservations must not be confused; this is the exact accepted C allowance, not a claim of 952 physically retained bytes.

The observed report agrees exactly: graph session-plus-plan and incremental totals are 227,148 rather than 226,196, and metadata is 51,247 rather than 50,295. Each difference is 952; all other fields in the full compared report are identical. L=224 is below the existing graph largest 49,167 and double-live maximum 58,804, so neither largest expectation moves.

The production replacement gate explicitly adds current and prospective graph session-plus-plan totals and both retained compiled models. Both plans have the same bank population; shortening the session ID affects canonical/model ownership, not this reservation. Therefore the existing double-live graph/model oracle increases by 2*C=1,904: 502,228 -> 504,132. This second correction is derived from the already observed per-plan component and inspected cap equation; execution has not yet reached that later assertion/admission, and no claim that it already passed is made.

## Exact allowed correction

Keep the existing independent primitive-owner style. A small private test helper may derive N from the fixture's nine tracks/eight lanes/three builtin stages and calculate C/L from primitive native layouts. Restate F as a two-pointer boxed trait-object footprint and B as F plus the boxed bool-slice footprint, or use one small test-local BankSlot field-list mirror with those same two owners. No dependency or Cargo change is needed. Do not invoke `GraphBankSlotResourceEstimate`, its fold, the production resource report or an observed difference as the oracle. Assert the frozen fixture's N/F/B/W and C/L values so this narrow native derivation remains explicit.

1. In `frozen_scratch_report`, add this independent C only to the three authorized graph fields, retaining their reviewed base literals or documenting the exact resulting literals above. Leave every other report field and the full-struct equality unchanged.
2. Append one positively charged row named as a runtime bank-slot coexistence reservation to `graph_owners()`, using the independent C. Append it after the existing rows so the existing `[5..13]` graph-metadata allocation authority retains its meaning. The existing clone into the prospective graph then charges C once for each plan automatically. Do not add another separate double-live charge.
3. Update only the two graph/model total expectations from 502,228 to `502_228 + 2*C` (504,132): `assert_effective_owner_mutations` inside `primitive_replacement_oracle` and the later `oracle.graph` assertion in the frozen exact test. The added row must participate in the existing omission and one-byte-miscount controls.
4. Preserve the existing eight cap rows and their exact/minus-one behavior, canary, atomic report, actual replacement/render and ownership destruction assertions. The graph row already consumes `oracle.graph` and will therefore exercise the derived 504,132/504,131 boundary without another literal cap change. Add a narrow assertion that independently derived L does not displace the existing graph-largest authority; keep 49,167 and 58,804 unchanged.

Source totals/overhead, effect state/scratch, builtin payload, C API retained storage, PCM, latency/tail, canonical document lengths/identities, all non-graph cap thresholds and existing allocator/lifecycle mechanisms remain frozen. No browser resource expectation is authorized by this ruling; any actual downstream mismatch must be separately observed and derived. Preserve the raw initial workspace failure unchanged.

## Objective gates and delivery

Run the existing exact test with nonzero selection in both profiles:

```
cargo test --locked -p capi --test resource_lifecycle external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps -- --exact
cargo test --locked --release -p capi --test resource_lifecycle external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps -- --exact
```

Each must execute one passing test, thereby reaching all eight cap rows and their existing negative controls. Run the existing complete `resource_lifecycle` integration target in both profiles (four tests each), strict affected C API all-targets Clippy, fmt/diff, and then resume the existing immutable delivery route on the identified integrated source. Capture exact source identity/commands/numeric statuses. No new test corpus or numerical sweep is required. A further discrepancy is a returned concrete finding, not authority for automatic additional repins.

The separate exact artifact-pin/current-consumer ruling remains valid; these test-only changes cannot justify any new artifact digest. Final integrated review, required CI and GitHub synchronization remain root's obligations. Read-only review: no builds/tests, timing, source/spec/Git/GitHub mutations performed. Only this requested temporary ruling was written.


## Browser resource integration and C API gate results

The exact C API oracle passes once in each profile; the complete lifecycle target passes all4 in each profile, strict CAPI Clippy and corrected diff check pass. Original EOF-whitespace failure remains preserved. Astra integration source review is PASS. The builder/static gates pass; the browser resource gate identified precisely three +204 graph rows and all26 rejection controls pass. Full direct-oracle comparison differs only in these rows; all PCM digests remain unchanged.

# Astra #511 browser resource integration ruling — approved, bounded

Reviewed clean frozen `e3cba8e853c86a8aa941cfa21470c6d9ae071a33` in `/home/bl/misofm/engine-slot-reservation`, the accepted reservation and earlier integration rulings, actual browser resource failure records, the separately captured successful direct oracle, the frozen browser session/expected document, and the relevant planner/layout/gate source. The ordinary consumer pipeline is terminal: builder 0, static checks 0, resource gate 1. This is an observed downstream numerical mirror discrepancy, not an inferred permission to repin other outputs or another production implementation attempt.

## Actual evidence and PCM ordering

`/tmp/511-qualified-resource.{command.json,log,status}` identifies this source and invokes `python3 scripts/check-browser-expected-resources.py --artifacts /tmp/engine-511-qualified`. The log names exactly three stale graph rows, each +204, and reports all 26 red mutation controls successful.

Independently hashed the published `/tmp/engine-511-qualified/miso-engine-v1-audio-worklet.simd128.wasm`: 2,693,746 bytes, SHA256 `eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4`, exactly the separately approved pin. No additional digest is authorized.

The gate collects the direct oracle through `MISO_ENGINE_WEB_ORACLE_PRINT=1` before `check_pins` compares resource rows. Inspection confirms that `direct-oracle.mjs` checks identity PCM against its unchanged native digest pin, command-timeline PCM against its unchanged native pin, and observed/unobserved PCM equality plus the unchanged native observation digest before it prints. Those checks precede the resource mismatch even though the later Python digest/full-document comparison follows the resource check and was not reached by the failing gate. This evidence establishes equality to the existing native pins; it does not pretend a fresh native PCM render ran inside that print invocation.

The additional `/tmp/511-integration-browser-direct-oracle.{command.json,stdout,stderr,status}` is an explicit clean-head print-mode invocation, status 0. Independently compared its complete recursive JSON to `expected.json.directOracle`: the only differences are the three resource fields listed below. All PCM digests, statuses, memory values, command results and observation values are identical. This resolves the full-document uncertainty left by the resource comparator's early failure without changing any oracle or pin.

## Independent component derivation

`hosts/host-web/tests/browser-v1/session.json` contains one track and no effects in any rack. The actual builtin planner groups each of post-input, post-fader and post-matrix into ceil(1/4)=1 padded bank on the shipped simd128 backend. Thus N=0 effect banks+3 builtin banks=3; later builtin pairing cannot reduce this pre-admission membership count.

For wasm32, the boxed stage's data/vtable pair occupies F=8 bytes. `BankSlot` has that owner plus a boxed bool slice, B=16 bytes. The selected four-lane mask has W=4 bytes. The frozen #511 calculation therefore gives C=3*(8+3*16+3*4)=204 and L=max(3*8,3*16,4)=48. This is the conservative named reservation, not 204 newly retained physical bytes. It adds only to graph metadata, incremental and session-plus-plan totals. L=48 is below the already pinned largest named allocation of 16,384, so that field stays unchanged.

## Exact authorization and gates

Approve only these three string-value replacements under `directOracle.simd128.resources` in `hosts/host-web/tests/browser-v1/expected.json`, plus the corresponding issue/commit/evidence derivation:

| Field | Existing | Approved |
| --- | ---: | ---: |
| graphSessionPlusPlanBytes | 29294 | 29498 |
| graphIncrementalPlanBytes | 29294 | 29498 |
| graphMetadataBytes | 3455 | 3659 |

Preserve every other expected value and byte authority: PCM/native digests, full document shape, source/builtin/effect/bridge rows, largest allocation, canonical/session fixtures, memory, protocol/status/timeline results and observation values. No production, dependencies, resource schema, gate scripts/classification, browser fixture, test filters or artifact pin change is authorized.

After this exact checkpoint, run the existing direct oracle without print mode against the same identified published module and corrected expected document; require its complete equality check to pass. Re-run the existing resource gate, including its 26 red mutation controls and native row-classification witness. Then finish the already approved hermetic worklet, npm installation, current three-browser qualification/self-tests and generated matrix checks. Reuse the accepted artifact when its source/pin/bytes are unchanged. Preserve the original failed resource record and successful pre-edit oracle capture unchanged; identify the corrected expected source and all terminal statuses honestly.

Further observed discrepancies require their own concrete ruling; this is no blanket numerical repin or new matrix. The existing immutable workspace/native/Wasm sequence, final integrated review, required CI and GitHub delivery synchronization remain root's obligations. Read-only review: no builds/tests, timing, source/spec/Git/GitHub mutations performed. Only this requested temporary ruling was written.

## Current artifact consumer qualification

Candidate eba52341 passes the complete corrected direct oracle, resource gate with26 rejection controls/native row witness, hermetic worklet, npm installation, all three browser qualification/self-tests and generated matrix check. Builder/static success on e3cba8e8 composes with unchanged production/pin/module bytes; module2693746bytes hashes to eb573b1e5fa083eb9d12f90a21de99536310d5c8379d5d2c671370a1dbfb32c4. Generated records change only candidate/module identity. Original mismatch captures are preserved. Immutable workspace/targets/native ABI and final integrated review/CI remain pending.

## Immutable integrated qualification PASS

Clean d1295506 passes the workspace (277 result blocks;1660passed/0failed/24ignored), supported scalar/SIMD Wasm checks, release native C API and ABI check. All five captured commands return0 and source remains unchanged. Artifacts/issue511-integrated-qualification preserves complete integration lineage, raw failures and final successes, commands, source identities, component rulings and current module identity. Final actual-PR Astra review and required CI remain pending; #511 is not yet delivered.

## Delivered — PR #515

Merged2026-09-06T09:16:15Z as107b9ed1803b8313e434821ec5bf49a178b6bb2f after Astra actual-PR PASS on fd6953774bc60c0cc0b15ef73cc72e0f3057a95d against a6a59030 ([review](https://github.com/misofm/engine/pull/515#issuecomment-5558248762)) and required qualification34023823998 SUCCESS. GitHub511 CLOSED. This delivers the runtime slot reservation prerequisite, not all of RT8 or an unmeasured speed improvement. #478 still requires fresh base review. Post-main qualification34024175768 is running and is not yet claimed green.
