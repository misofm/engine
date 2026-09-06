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

The missing reservation is now owned by #511 (`511-runtime-bank-slot-reservation.md` on its dedicated branch). It reserves C=N*(F+3B+3W) and L=max(NF,NB,W) using actual combined prepared/planned bank populations before cap admission, with independent ownership and transactional evidence. Luna attempt1 did not satisfy its full gates; Astra issued one consolidated FAIL and Sol attempt2 is active. #478 remains queued with zero implementation attempts consumed until #511 is delivered and its actual integrated base is reviewed. #509 and #512 maintenance are delivered independently; neither resolves this accounting prerequisite.
