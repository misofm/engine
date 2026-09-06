Prepare immutable rack slot activity for block dispatch

# Ready-to-number RT-8: prepare immutable slot activity for BankChain dispatch

Queued Class-A product scope for audit #349 RT-8, not implementation authority. Inspected stable delivered660fce8f source in engine-475-plan. #443 remains active; existing #238/#463/#475 priorities are unchanged. No builds/tests/timing or repository/GitHub changes performed.

## Source premise and ownership

rack::BankSlot1150 publicly exposes stage and active_lanes. BankChain::new1489 consumes Vec<BankSlot>, checks exact mask width and subset of chain activity, then owns slots privately as Box<[BankSlot]>. No public method returns mutable slots/masks. Later self.slots mutations invoke stage operations/counters/observation disarming, not mask writes. Arm-fold/aux/mono methods change separate chain fields. The mask's contents therefore become structurally immutable after validated construction even though caller-owned BankSlot is publicly mutable before transfer.

Three run-path checks at1785,1904,1929 recompute mask.iter().any: begin_block, prefix/ordinary processing and collapsed seam-side processing. This is distinct from live lane symmetry, current observation state and bypass activity. Do not cache those changing facts or expand RT-8 into RT-6.

## Frozen smallest representation

Preserve BankSlot's public fields, constructor signature and caller behavior. Use ONE private prepared-slot type inside BankChain with the existing Box<dyn BankStage> and an active-lane u8 mask, derived only AFTER original width/subset validation. Current legal BankWidth has4/8 lanes; this is a lane-mask representation, not MAX_TRACKS or a slot-count limit. Keep arbitrary slot counts. All-false mask is0 and must retain its original stage owner even though its begin/process calls remain skipped.

Pack the existing lane bits at preparation; do not add a caller-trusted nonempty boolean, mutable cache API, second parallel allocation or generic mask framework. This compact private mask is also the cached nonempty fact (`mask != 0`) used at exactly the three target call sites. The sole remaining per-lane slot-mask read in lane_symmetry uses the corresponding guarded bit; chain active checks still reject out-of-range lanes first. Compute collapse_prefix_of against the validated original slots BEFORE conversion, so its existing whole-mask equality/seam-side/support rules do not change. Do not discard inactive stages, move the collapse boundary, reorder slots or rewrite any stage math.

This representation is chosen over adding bool to a private wrapper: appending bool to the current two-fat-pointer slot typically enlarges its allocation; packing the existing at-most-eight lane booleans into the private mask avoids another retained allocation. No unsafe pointer tagging, alignment tricks or relying on enum niche layout is authorized.

## Exact allowed implementation/proof paths

crates/rack/src/lib.rs: private prepared slot, conversion after validation, the three existence checks, the one per-lane bit lookup, existing inline fixtures and layout/mechanism proof. crates/rack/tests/console_bank.rs and mono_reengage.rs: only the directed existing queue/mono fixtures needed below. Their existing MUTATIONS.md and numbered evidence may record the one mechanism control.

Resource verification may use the existing graph/runtime construction and CAPI resource tests READ ONLY initially; source changes there require a precise demonstrated affected estimate, not permission for generic accounting work. No public BankSlot API, graph scheduling, producer/record admission, BankStage vtable, Cargo dependency, DSP kernel, helper, CI, runner or corpus edit. Actual current-base approval before assignment must confirm this conversion still covers every slot-mask use.

## Accounting is not free

Independently restate both layouts and record target Layout sizes/alignment and actual constructor allocations, rather than assert a guessed byte saving. On ordinary64-bit layouts, the current BankSlot consists of a two-word trait-object Box plus a two-word slice Box (32 bytes), while the proposed private stage-plus-u8 typically occupies24 bytes after padding. On wasm32 the corresponding typical sizes are16 and12. These are predictions to verify, not measured evidence or ABI promises.

Old retained slot storage is Layout::array::<BankSlot>(S) plus the S width-sized bool allocations. New retained storage is Layout::array::<PreparedSlot>(S), retaining exactly the same stage boxes. Conversion may temporarily hold the caller's old Vec allocation, remaining old masks and the new output array concurrently. Record its actual peak and largest individual allocation; do not claim zero preparation allocations or infer peak reduction from retained reduction. No rendered allocation/free is allowed.

BankChain's field remains a fat Box slice; its own and RuntimeUnit's layout should remain identical but must be checked. Existing graph/CAPI bank resource rows describe bank processors/membership/scratch; the examined CAPI oracle does not have a named BankSlot mirror. Do not subtract predicted savings from unrelated bank processor rows or blanket-repin totals. Keep current resource caps conservative; independently demonstrate the new retained footprint/largest allocation does not exceed the prior accounted bound, and compare actual preparation peak against the relevant existing bound. Preserve current bank CAPI exact-cap/one-below regressions if their defined totals are unchanged.

If inspection/measurement reveals an existing unaccounted slot allocation or a new peak that the current capped estimate cannot cover, stop at that concrete fact BEFORE accepting implementation. Root must add the smallest explicit estimator/test seam or number a separate accounting prerequisite; do not silently waive the bound or smuggle a general resource redesign into RT-8. This contingency is an acceptance stop, not implementer choice between multiple designs. No runtime resource/speed claim is authorized before the actual layout proof.

## Finite discriminating product gates

1. Validate unchanged rejected shapes before conversion: wrong mask width, slot active outside chain mask, all-inactive chain. Preserve zero-slot legal chains and all-false slots. Full and partial4/8-lane banks with empty, one-bit/holey and full slot masks must pack exactly; caller mutation before ownership transfer must be reflected in the prepared value. No public source compatibility change.
2. One compact old-reference stage trace fixture records begin_block/process/process_mono order, first_sample, frames, queued record effects and errors. Compare old mask-any oracle versus cached dispatch for leading/middle/trailing inactive slots and active slots around the collapse prefix. Empty slots must neither drain queues nor execute arithmetic; retain their owned stage/drop lifetime until normal off-render retirement. All begin calls still precede gather/processing, and failure stops at the same original slot.
3. Reuse existing mono_reengage/console_bank tests for actual queued control transitions and collapsed→dual→collapsed recovery. Check PCM bits, private stage state, channel-copy/collapse transitions and active/inactive lane sentinels, not just call totals. Preserve live witness queries; the new immutable mask must not suppress a changing stage's symmetry/observation/bypass state.
4. A test-only mechanism discriminator verifies the actual three dispatch sites use the prepared mask without scanning slot masks per block. ONE actual temporary restoration of the old per-block existence computation must fail the SAME excess-work/selected-mechanism assertion while semantic PCM/trace assertions remain equal. Do not create a public runtime counter, benchmark-only implementation or a large mutation campaign. Retain diff, original failure and restored success.
5. Installed existing allocation/free audit with positive liveness: repeated actual full/partial ordinary and collapsed/fallback rendering remains zero allocations/frees. Preparation layout/peak proof is separate from this rendered zero result. Reuse an existing allocator harness, no new allocator or framework.

## Proportional execution and delivery

After numbering/actual-base approval, source qualification commands include cargo test --locked -p rack --lib, --test console_bank and --test mono_reengage, and corresponding --release forms with actual nonempty test names/counts. Existing affected graph/rack integration identity, strict clippy/fmt, realtime/lane/workspace policy and allocation gates remain. Then root freezes immutable candidate for required workspace/supported target and any actually affected artifact/static/browser qualification. Source evidence must not claim current native/Wasm instruction savings from source shape alone.

No timing is authorized by this issue draft. Descriptive measurement or broader RT-6 mask publication remains a separate issue, with its own workload and authority; do not add a runner or tune this change against a number. One Luna pass then Sol2/3 as needed, each one coherent checkpoint/verdict, hardstop/rescope after3. Root owns checkpoints, remote synchronization, actual PR review and required CI before closure. Closing this child resolves RT-8's three invariant scans only, not other rack/RT audit findings.

## Numbered queued scope

GitHub #478 matches this title and stateless body. The planning branch is based on delivered main `fa3485c6bb1a69e6dd01df734a1ad9c945964715`. Root verified the inspected rack source, relevant integration tests, resource test inputs, Cargo/configuration and fixtures unchanged from the Astra-inspected660fce8 base. Numbered scope approval is still required before assignment. This queued issue does not displace #443 or the already approved queue and authorizes no implementation or timing. The preparation-accounting acceptance stop remains explicit.

## Astra numbered scope approval

# Astra #478 numbered scope review — PASS

Reviewed planning head de9564c9a76c660c979d68e1569f864eb8564147 in engine-rt8-plan, based on delivered fa3485c6. The sole delta from that main is .github/ISSUE_SPECS/478-prepared-rack-slot-activity.md. The complete /tmp/astra-rt8-current-scope-brief.md is preserved verbatim, followed by the queued numbered record. Live GitHub #478 is OPEN, exact title “Prepare immutable rack slot activity for block dispatch”, and body matches the local spec. Read-only diff confirms relevant rack/graph/resource/Cargo/configuration/fixture inputs unchanged from inspected660fce8.

PASS for numbered queued scope. This preserves public BankSlot and constructor validation, freezes private packed-mask preparation for the three invariant activity checks, and leaves live stage symmetry/observations untouched. Old-path trace/PCM/state/queue/order/mono transitions, actual mechanism control, installed allocation audit and explicit retained/peak/layout accounting remain required. No additional public caller-trusted flag, unsafe representation trick, resource subtraction from unrelated bank rows, new runner or timing authority is introduced.

Preparation allocation accounting remains an explicit acceptance stop if the new conversion peak exceeds an existing accounted bound or exposes omitted storage. Root must resolve that precise estimator/test seam before source acceptance; the issue does not call the cache free. No implementation is authorized by this review now: preserve active443 and queued238/463/475 priority, and freeze/recheck the actual implementation base when assignment is reached. No tests/builds/timing or repository/GitHub mutations performed.

## Current implementation-base readiness

Root integrated delivered main1543c4c2 after #496/PR508 and #506/PR507, with both closure records. The entire rack crate remains byte-identical to prior approved14f46280; downstream builtins/graph/compiler/tooling/Cargo test inputs have advanced and are not described as identical. Earlier active/queued references are historical: #443/#238/#463/#475/#496 have delivered. This is the next runtime feature after #496, independent of #509 maintenance. Actual-base Astra review remains required before implementation; the existing resource retained/peak/layout acceptance stop remains explicit, and no timing is authorized.

# Astra #478 queued readiness — applicable, still queued after #496

Read planning head 14f462809d5fb44d007ff8cf953d165abe87be8a in engine-rt8-plan and explicit delivered main ad00d16b8ef8e3aa5ba4c406d00db4c62ff311b5. Live #478 remains OPEN with the matching title. The frozen approved implementation design still applies; this report does not authorize implementation or bypass #496/#503 sequencing.

The entire rack crate, Cargo.toml/Cargo.lock and .cargo inputs are byte-identical from scoped fa3485c6 to ad00d16b. Consequently the original premise and line references remain exact: public BankSlot1150, privately owned slots1409, constructor mask validation1496, collapse-prefix1562/1575, one lane-symmetry mask read1655 and the three repeated any scans1785/1904/1929. Masks remain immutable after validated ownership transfer. No new slot-mask reader or writer invalidates the packed private u8 choice. Existing rack console_bank/mono_reengage fixtures are unchanged.

Concrete stale context is the old queue prose: #443/#238/#463/#475 have since delivered, and root now queues this after #496. Update that status/current-base record when assignment is reached rather than rebriefing unchanged product scope. The planning branch itself remains based on fa3485c6; it must integrate actual then-current main and receive an implementation-base check before Luna1.

Downstream graph/program/runtime, graph-compiler and builtins-compiler allocation tests have changed since that old base through delivered scalar pairing, scheduling-oracle and control-plane maintenance. Do not carry forward a claim that those whole dependency/test trees are byte-identical. The specifically referenced CAPI resource_lifecycle.rs remains unchanged. None of the observed downstream changes alters rack's immutable-mask premise or authorizes subtracting cache savings from unrelated bank/scalar processor accounting.

Retain the existing precise accounting stop: verify actual prepared-slot retained layout, conversion peak and largest allocation against the applicable current bound; any uncovered cost needs the smallest explicit ruling before acceptance. This was already part of the approved scope, not a newly added gate. Reuse the current existing allocation harness when assignment is reached; do not blindly reuse a process-wide counter interval as render-thread attribution in light of #503. No new allocator or harness is required or authorized here.

The frozen three-site mechanism control, old-reference order/error/ownership trace, actual queue/mono PCM-state and liveness proof remain sufficient finite scope. No new matrix, architecture change, benchmark authority or instruction/cycle claim follows from readiness. No source/spec/Git/GitHub mutations, tests, builds or timing performed.

## Actual-base accounting prerequisite; implementation remains unassigned

# Astra #478 actual-base review — accounting prerequisite required before Luna

Reviewed clean exact heade147263a0d5e61550688f162f8050e1c7d7ed640 in engine-rt8-plan, integrated delivered main1543c4c2. Only #478 readiness and #496/#506 closure records differ from main. Rack implementation remains byte-identical to approved14f46280; public BankSlot, immutable ownership, three any scans and the one per-lane read remain applicable. The proposed private packed mask itself needs no redesign. This is not yet approval to assign implementation because the frozen resource/measurement prerequisite is not concretely established.

## Actual ownership and missing bound

Graph runtime.rs1413–1425 is the production constructor seam: bank_chain consumes Vec<Box<dyn BankStage>>, creates a Vec<BankSlot>, clones the active bool slice per slot, then calls BankChain::new. Rack lib.rs1489–1538 validates those masks, computes collapse_prefix, consumes slots.into_boxed_slice and allocates full-bank staging planes. There is no cap/resource argument in BankChain::new and no retained/peak estimator there.

The named surrounding resource quantities do not establish a cap over this conversion merely because they mention banks. graph::GraphResourceEstimate describes effect_bank_metadata_bytes as metadata retained BEFORE render binding. builtin_bank_resource in builtins-compiler/src/lib.rs1174–1240 charges prepared processor payload, member arrays/strings, GraphPreparedBuiltinBank metadata and AoSoA scratch; it has no explicit BankSlot array/per-slot bool-mask/runtime-chain conversion term. graph-compiler/estimate.rs graph_metadata_bytes sums graph nodes/edges/schedule/levels/buffers/timing/IDs; it likewise does not identify this slot storage or a preparation live peak. CAPI resource_lifecycle is not a BankSlot mirror. Do not subtract predicted savings from any of those fields or claim their current total is a proven post-bind peak bound.

This establishes an unaccounted/at least un-attributed runtime slot-storage concern at the exact seam the frozen scope told us to stop on. It does not prove every whole-plan cap undercounts by a particular number; conservative surplus elsewhere may exist, but no explicit applicable inequality has been supplied. It must be resolved by a narrow accounting ruling before implementation, not by declaring the cache free or inventing a guessed bound.

## Finite reusable proof seam and its limits

crates/builtins-compiler/tests/allocation_tracker.rs already has a thread-local ArmedGuard and TrackingAllocator, alloc/alloc_zeroed/realloc/dealloc hooks, positive alloc/free liveness, and actual prepare/bind scopes (actual_scalar_prepare_and_bind_retain_the_charged_owner_layouts around349), plus audit_graph_render around154. Reuse that existing allocator if a bounded test-path amendment is approved; no new allocator or public rack diagnostics are needed. It is currently outside #478's allowed source paths, so do not edit it implicitly.

Its TestPhaseTwoAllocationSnapshot collects total allocation bytes, largest and grouped allocation/deallocation layouts. Those aggregate counts are not a timeline of live bytes and cannot by themselves certify peak coexistence of caller Vec, remaining masks and new packed-slot storage. A narrowly scoped test-local current/peak byte observation using the existing hooks, or an explicit conservative simultaneous-allocation inequality validated by observed allocations, must be selected before assignment. Start the measured owner scope before constructing slot/mask ownership and include conversion, then observe retained state and off-render release; bracket unrelated buffers/fixture setup consistently. Include caller Vec capacity, zero-slot, full/partial widths and arbitrary slot-count scaling in the formula, not an arbitrary global maximum. Do not treat process-global counts as thread attribution.

The existing graph/tests/rt1_direct_bank_alloc.rs uses the shared armed allocator for actual ordinary/folded graph renders and can remain a render-zero regression. Its hand-written zero resource estimate is a fixture constructor, not a preparation budget oracle. The builtins-compiler existing actual queued render audit is another reusable zero/liveness seam; neither substitutes for peak accounting.

## Required root decision before assignment

Freeze the smallest explicit accounting contract for runtime BankSlot/mask ownership and conversion peak: identify the existing cap/charge that truly covers it with an independent formula, or number a bounded runtime-bank accounting prerequisite if the charge is absent. Select the exact estimator/consumer fields and test path under that separate ruling before allowing a dependent change. A general resource redesign, public cache flag, production graph scheduling change or blanket CAPI repin is not authorized. A second allocator or general measurement framework is unnecessary and outside scope.

Once that prerequisite is resolved, preserve the approved packed-mask product and its old-reference order/error/ownership, PCM/queue/mono, three-site mechanism and zero-render gates; there is no need to rebrief these unchanged parts. Keep #478 queued rather than consume Luna1 on an unresolved bound. #509 independent tooling remains unaffected.

Read-only source inspection; no tests/builds/timing or source/spec/Git/GitHub mutations performed. This report deliberately does not supply guessed native/Wasm allocation measurements.

Root preserves the existing packed-mask design and its frozen accounting acceptance stop. No Luna attempt has been consumed. Astra is scoping the smallest separately numbered accounting prerequisite with exact owner reservation/bound and existing test-local allocation seam; no generic allocator or resource redesign is authorized by this queued issue. Independent #509 maintenance continues.

## Numbered accounting prerequisite

#511 delivered through PR515/main107b9ed1 after final Sol3 source PASS, bounded integration acceptance, exact-PR Astra PASS and required qualification34023823998 SUCCESS. Post-main34024175768 is also SUCCESS. Its C=N*(F+3B+3W) and L=max(NF,NB,W) reservation now governs this change; native CAPI and browser numerical mirrors remain frozen consumers. Zero #478 implementation attempts were consumed before this fresh-base approval. Earlier queue/prerequisite-stop prose is historical and is superseded by the authoritative Luna1 brief below.


## Authoritative delivered-base amendments and Luna attempt 1

# Astra #478 delivered-base scope review and Luna attempt 1 brief — PASS

Reviewed clean `eadc52f4` in `/home/bl/misofm/engine-rt8-plan`, merge `b232922f` of delivered main `107b9ed1`, the complete numbered #478 spec, earlier implementation-base stop and queue-readiness reviews, delivered #511 accounting/construction/evidence paths, and current rack code/tests. The entire rack crate remains byte-identical to previously scoped `14f46280`. Against delivered main the only local differences are #478's accumulated planning record and #511 closure documentation. The reported #496/#506 documentation conflict introduced no rack/runtime change.

The missing named reservation that blocked assignment has now been delivered by #511/PR515. Its production source and accepted resource contract are present in this exact base. PASS to activate the first #478 implementation attempt after root records/synchronizes the amendments below. No #478 attempt was consumed by its prerequisite stop. Root has reported PR515 required CI success and remote #511 closure; its post-main run remains separately monitored, and this review does not claim that pending result passed.

## Smallest closable product

Replace only BankChain's three invariant per-slot nonempty-mask scans with a prepared private u8 activity mask. Preserve public `BankSlot { stage, active_lanes }`, `BankChain::new` inputs/errors, original width/subset/nonempty-chain validation, stage ownership/order and live symmetry/observation/bypass behavior. Keep arbitrary slot counts: the eight bits encode bank lanes, never slots or tracks.

Use one private unversioned prepared-slot type containing the original `Box<dyn BankStage>` and u8. Validate the complete original inputs first, retain scratch clearing/full-bank decisions, and calculate `collapse_prefix_of(&slots, &active)` on the original validated public slots before conversion. Convert in order into a separately owned vector explicitly requested for S initialized slots; do not grow it or rely on a capacity-opaque iterator collect. Finish as a boxed slice. Pack each bool at its original lane index, release its old mask off render, and retain every stage including all-false slots. No caller-trusted boolean, extra persistent parallel mask/array, mutable cache interface, pointer tagging or unsafe layout trick.

Change exactly the nonempty predicates currently at rack lib.rs1785,1904,1929 to prepared-mask nonzero. Change the slot-specific lane lookup at1655 to a guarded bit read. The existing chain-level active.get guard must reject lane indices outside the chain before any shift. All other slot operations remain stage calls, and all chain/aux/fold masks and live witness decisions remain unchanged. `collapse_prefix_of` retains its original public-slot mask equality and seam rules. Inactive slots necessarily prevent the all-masks-equal collapse prefix where they did before; do not manufacture an impossible collapsed chain with an empty slot to satisfy a test case.

This is one useful bounded rack feature plus its necessary existing ownership-oracle adaptation. It does not own a generic allocator/harness improvement, target-matrix expansion, performance framework, new DSP fixture corpus or unrelated queue rewrite. No timing or instruction/cycle improvement claim is authorized.

## Exact path amendment

Production remains solely `crates/rack/src/lib.rs`. Its inline tests may inspect the private representation and contain finite trace/layout/mechanism evidence. Existing directed fixture edits are allowed only in `crates/rack/tests/console_bank.rs`, `crates/rack/tests/mono_reengage.rs`, and their existing `MUTATIONS.md`.

Approve these two necessary narrow additions to the old path freeze:

- `crates/builtins-compiler/tests/allocation_tracker.rs`: adapt the existing `actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation` to the new actual allocation/release shape, keeping its installed thread-local allocator, byte-counter liveness, paired/unpaired graph observations and repeated zero-render checks. No replacement allocator or changed old phase-two realloc semantics.
- `crates/graph/src/runtime.rs`: only the already feature-gated #511 construction adapter/fact descriptions and test-only layout facts, if needed to identify its incoming public slot storage versus final private retained storage honestly and expose BankChain/RuntimeUnit layout sizes. No graph production constructor/scheduler/accounting change. Existing graph exports may be reused; no new public production diagnostic or rack Cargo feature is needed.

The numbered spec and focused evidence are the other permitted paths. Graph/graph-compiler estimators, `crates/graph/src/lib.rs`, CAPI numerical mirrors, browser expected rows, Cargo/lock/config, ABI, DSP, scripts/gates and fixture identities stay frozen during implementation. Existing graph/CAPI tests are regression consumers, not blanket permission to edit them. Any actual delivery artifact mismatch follows the separate observed-pin/current-consumer ruling route later.

## Delivered reservation and finite physical proof

Keep #511's C=N*(F+3B+3W), L=max(NF,NB,W), combined N and all admission totals unchanged. F is the original boxed stage, B the public BankSlot, W the maximum actual bank mask; define P as the new private slot's actual target layout. Establish P<=B and destination capacity<=S<=R<=N. The original graph constructor now explicitly bounds the public slot capacity by S; the incoming stage-vector capacity is bounded by R. Do not confuse stage capacity with post-pairing S.

A sufficient conservative graph-component bound is:

- New retained private slots plus chain masks: N*P+N*W <= N*(B+2W).
- Conversion, allowing incoming stages, old public slots, private vector plus a possible boxed-slice destination, and old slot/chain masks together: N*F+N*B+2*N*P+2*N*W <= C because P<=B.
- Each stage/public/private destination request and mask remains <=L.

Some named terms do not coexist in the actual ordered constructor; this conservative bound deliberately avoids a false process-wide peak claim. If implementation creates another destination, grows capacity, or cannot establish these inequalities, return that precise obstacle before acceptance. Do not consume or reduce #511's extra headroom in unrelated owners or lower resource totals to claim savings.

A legal zero-slot direct caller still owns its chain mask and scratch. Its S=0 control must prove zero private-slot allocation and correct identity/lifetime behavior; do not assert those caller-owned bytes fit #511's N=0 graph-bank component. A graph with no prepared/planned memberships creates no such bank chain. This distinction preserves both zero-slot public compatibility and the unchanged zero-population reservation.

`BankChain::new` also accepts arbitrary caller Vec capacities. Preserve that API without inventing a global cap for external callers. Its graph-admitted path has the #511 exact public-capacity premise above; tests should separately show spare-capacity public inputs still behave correctly, and attribute their input capacity explicitly rather than pretending N-based graph admission covered arbitrary caller storage.

Use the existing ragged identity-stage construction interval to identify every requested and freed layout. A native three-slot example is expected to replace retained public96 plus three8 masks with private3*P, while keeping the original8 mask and two independently identified scratch planes; these are predictions, not measured facts. Update the old exact request/free expectations from actual attributable new events, not by removing assertions. Seed preexisting ownership correctly, preserve checked failure flags, distinguish scratch/stage owners, assert no unexplained requests or releases, and verify zero closing balance after destruction. The zero-sized identity stages and ragged mask avoid unrelated stage payload and tiled staging allocations in this interval.

Cover W4/W8 and a small fixed set S=0,1,3,9 so zero, multiple slots and slot counts greater than mask width are represented without a sweep framework. Full-bank behavior may be established in semantic/render fixtures; its tiled staging stays a separate owner and is never charged to this component. Record native actual P/B/F sizes/alignment and array requests. Assert target-layout P<=B at compile time so the existing Wasm build also checks the real compiler-selected layout, rather than calling predicted wasm32 P12/B16 measured evidence. Retain source-level before/after BankChain and RuntimeUnit layout facts on the delivery target; their boxed-slice field footprint should be unchanged. A layout discrepancy is a returned fact, not permission to repin bank processor rows.

## Frozen finite semantic and mechanism gates

Freeze these new inline test identities before implementation:

1. `tests::prepared_slot_activity_preserves_shape_bits_and_ownership`: W4/W8 validation failures remain Shape before conversion; legal zero slots, empty/one-bit/holey/full slot masks, caller mutation before transfer, arbitrary slot count and spare input capacity retain the original behavior. Check exact prepared bits, lane-out-of-range decline, unchanged collapse-prefix decisions and inactive-stage lifetime/drop behavior. Assert private layout <=public layout.
2. `tests::prepared_slot_activity_preserves_dispatch_trace_and_errors`: a compact test-local old mask-any reference over the same prepared input descriptions compares begin/process/process_mono order, samples/frames, stage state/PCM bits and failure stopping position. Include ordinary chains with leading/middle/trailing inactive slots, and separately legal collapsed prefix plus seam suffix. All eligible begin calls still precede processing; an inactive stage neither drains nor computes, but remains owned. Existing real console/mono fixtures supply queued-control and recovery behavior; do not create a second production executor.
3. `tests::prepared_slot_dispatch_uses_constant_activity_checks`: exercise actual ordinary and legal collapsed BankChain::run calls so all three production guard sites are reached. Use narrowly test-only, allocation-free accounting of actual activity-predicate work, scoped independently of required live symmetry reads. Assert prepared nonempty queries inspect no per-lane activity sequence in render, while semantic trace/PCM checks run before the work assertion.

Freeze exactly one actual mechanism control: temporarily replace the three prepared nonzero guard computations in the production run path with the old bounded per-lane nonempty computation. Since persistent bool slices are deliberately removed, it may scan the original lane values decoded from the packed byte (or reconstruct a fixed stack bool array) in original lane order. This is the representationally adjusted restoration of the removed per-block scan, not a claim to retain the old heap representation. The actual loop's lane inspections must drive the test-only work observation; do not increment an arbitrary mutant marker or compare two disconnected helpers. Use the same frozen mechanism test, which must reach equal semantic assertions and then fail its unchanged excess-work assertion. Preserve exact mutation diff, failing command/status/assertion and restored-source passing result. One mutation spanning the three sites suffices; no separate campaign is requested.

Reuse existing console_bank lane-command/bypass/partition fixtures and mono_reengage's forced-off, bypass, rejected recovery and earned-agreement cases. Their PCM/state/channel-copy transition checks remain load-bearing. Add only a directed missing case in those files if necessary; do not cache changing stage facts to simplify tests. Existing allocator liveness and actual ordinary/folded/full/partial graph render audits remain mandatory and separate from preparation accounting.

## Exact execution and checkpoints

Before source changes, capture the small applicable layout baseline using the allowed existing test seam if needed; do not introduce a timing or reusable layout tool project. Root must first append/synchronize this brief and freeze the three test names/feature commands in #478.

For the first coherent source tranche, run each of the three named inline tests as `cargo test --locked -p rack --lib TEST_NAME -- --exact`, with one actual selected test, plus the adapted physical exact command:

```
cargo test --locked -p builtins-compiler --features test-support,graph/test-support --test allocation_tracker actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation -- --exact
```

Once those are green, pause for root's exact-path checkpoint before more implementation. Repeat the three inline exact tests and physical exact test with `--release`. Execute the one mechanism mutant once on the frozen debug exact test, preserve its intended failure, restore accepted source and prove that exact test passes again. Do not tune or rerun the mutation to accumulate a campaign.

Run these affected suites in debug/release with actual nonzero counts:

```
cargo test --locked -p rack --lib --test console_bank --test mono_reengage
cargo test --locked -p builtins-compiler --features test-support,graph/test-support --test allocation_tracker
cargo test --locked -p graph --test rt1_direct_bank_alloc
cargo test --locked -p capi --test resource_lifecycle
```

Retain the #511 accounting exact regression unchanged (`cargo test --locked -p graph-compiler --lib tests::runtime_bank_slot_reservation_is_published_and_capped_transactionally -- --exact`) and its full graph-compiler lib suite in debug; release coverage follows the same existing affected route, without a new matrix. Strict affected all-targets Clippy must cover rack plus the graph/builtin test-support integration; keep the previously restored isolated `builtins-compiler --features test-support` consumer compilable. Run fmt/diff and existing rack/graph/realtime/lane/workspace policies, explicitly through bash. Record source identity before every gate; include actual selected counts, individual statuses, candid failures and target-layout limits.

After one coherent Luna pass and evidence checkpoint, stop for one consolidated adversarial verdict. Root owns all Git/GitHub changes and pushes; no overlapping implementation tranche may begin before its coherent checkpoint. Sol attempt2/3 only if that verdict requires them; after three failed implementation attempts total, preserve evidence and rescope once. #478 is the sole launch-critical implementation WIP; independent #514 delivery may proceed on separate inputs.

After source PASS, root performs the established immutable workspace/supported-target/native ABI and actually affected ordinary artifact/browser qualification, followed by actual-PR review and required CI. Broader performance investigations and unrelated ownership concerns belong to successors. Do not keep the product open for a second harness or an expanded corpus.

## Numbered-spec amendments root should adopt explicitly

Append this delivered-base approval as the authoritative current decision; preserve earlier queue/readiness/prerequisite-stop text as dated history. Replace the current-status paragraph claiming #511 Sol2 is active with: #511 delivered through PR515/main107b9ed1, source/core and bounded integrations accepted, zero #478 attempts consumed, and this exact-base review activates fresh Luna1 after synchronization. Distinguish root's still-monitored post-main job from already successful PR qualification.

Amend the exact-path section with the two narrow existing-test/seam allowances above. Supersede the obsolete claim that no named slot reservation/CAPI mirror exists: #511 now supplies C/L, and its native CAPI/browser numerical mirrors are frozen consumers. Replace the ambiguous demand to measure a global actual preparation peak with the explicit attributable conservative coexistence inequality backed by actual requested/free layouts, exactly as #511 approved. Freeze the three test identities and the representationally adjusted one-scan mutation above. Preserve public API, masks/validation/prefix, arbitrary slots, no timing, no numerical subtraction and the precise uncovered-capacity/layout stop.

This review supplies source approval and a concrete bounded brief, not measured P values, a test PASS, post-main CI success or repository mutation. No builds/tests, timing, source/spec/Git/GitHub mutations were performed. Only this requested temporary report was written.

Root adopts all explicit path, ownership-bound, test-identity and mutation amendments above. The delivered-base report's pending post-main reference is historical:34024175768 has since completed SUCCESS and its closure evidence is carried here. The graph component uses the stated conservative coexistence inequality plus actual allocation/release attribution, not an unmeasured global preparation peak. Existing graph test-support adapter and builtin allocation fixture are the only added test paths. No #511 numerical totals or production graph accounting may change. Luna attempt1 is now active following remote spec synchronization; root retains all Git/GitHub ownership.
