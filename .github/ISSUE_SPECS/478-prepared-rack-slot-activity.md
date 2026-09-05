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
