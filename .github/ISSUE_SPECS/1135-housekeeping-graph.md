# Housekeeping: graph

## Authorized scope and smallest closable slice

Review the complete `crates/graph` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/graph` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker B, GPT-6.1 Sol xhigh, 2026-10-01

Read the entire manifest, production source (`lib.rs`, `program.rs`, `runtime.rs`), unit-test source, three RT integration files and existing mutation evidence. Reviewed current related scopes #1154, #932, #931, #899, #892, #887, #967, #968, #969, #952, #1074 and #1078, plus applicable graph entries in the broader #349/#559/#560 trackers. No dependency, public API, algorithm, arithmetic, feature or generated-fixture change. Checkpoints: `55544e7d` (production/test cleanup), `2e857382` (reference helper without temporary copies), `fa96fa5f` (current source/inert/dead oracles). Net Rust change against `8edfcf80`: **115 additions, 402 deletions, −287 lines; two test functions removed**. No new test family or harness.

#### Five-axis findings

| Axis | Change and retained contract |
| --- | --- |
| Repetition / LoC | Three metadata-addition wrappers share one private transaction. All three checked sums complete in their original overflow order before any field assignment; largest-allocation update and wrapper APIs remain identical. Deleted historical reduction bodies and historical output hashing; current independent scalar/analytic and current executor references remain. |
| Test value / cost | Removed two Rust-layout history tests and 15 pre-change PCM/meter hashes. Preserved every numeric generator, hostile value, width, directed case, fault, RT counter and diversity discriminator. Inert cases now render the same fixture through the current executor with every unit dispatched and compare all host words and meter frames. This runs a second current arm rather than hashing once; it earns its cost by checking the actual skip contract without historical bytes. |
| Copies | Structural validation and bank attachment borrow node/level/member keys, flattened schedule entries and position keys. Ordered iterator equality retains cardinality, membership and order, including trailing entries. Necessary owned bank-member variants, published owners, rollback snapshots and runtime lifetime data remain. The independent scalar reduction reads all inputs for one frame before writing that frame, with no temporary Vec or copy-back. |
| SIMD | Removed only `FoldCohort` invariants already proved by its sole public constructor and private immutable fields: nonempty/≤8, unique lanes, stride/capacity. Kept lease frame count, writable master, fold-table and store-order checks before writes. Routes/reductions/transposes, scalar tails and W4/W8 arithmetic are untouched. Native release assembly retains packed `vmulps`/`vaddps` and scalar `vmulss`/`vaddss`; changed `fold_cohort` has no memcpy/memmove/memset call or tail jump. Existing reduction memcpy/memset tail jumps implement the one-input copy/zero-input fill. Resident fold has no such copy calls or tail jumps. No additional ISA algorithm or timing claim. |
| Data structures | Borrowed ordered sets/maps remove identifier ownership during validation without changing lexical diagnostics or deterministic schedules. Runtime keeps dense interned unit/buffer arrays and bounded dispatch/source tables. Broad op-storage flattening, scatter-redirect asymptotics and resource accounting redesign remain with existing owners. |

#### Removal and replacement coverage

| Removed restriction / body | Surviving owner and why it suffices |
| --- | --- |
| `the_observation_lane_costs_one_nullable_pointer_in_the_live_control_effect` | Current derived observed flags, ordered resident/lazy observation dispatch, permanent-skip vs unconditional PCM/trace comparison, and live-control boundary/bypass/capacity tests defend reachable observation/control behavior. An old Rust private layout is not a wire or corpus contract. |
| `no_reported_runtime_byte_moved` | Current metadata charge/overflow/ceiling tests and actual prepared-plan zero allocations/frees remain. They defend claimed resource admission and render ownership; they do not pretend to detect every uncharged future heap field (#1074 owns that gap). |
| Old block/per-vector reduction implementations and repeated old-output arms | Every-width hostile directed comparison and seeded fan-in comparison retain independent ordered scalar definitions, exact finite/NaN words, tails, aliases, repeated IDs, source preservation and nonvacuity controls. Fixed association counterexamples retain independent analytic expected values. |
| `old_reduce_plane` callers | Repeated-silence/self/unrelated-buffer case and both real graph folded-cohort/epilogue cases now use `reference_reduce_plane`: independently routed inputs followed by scalar left-to-right addition, with reads completed before the frame's write. It never calls the production reducer. |
| Nine `SOURCE_SHAPES` hashes, two dead-claim hashes and `SourceRun::digest` / `inert_fnv` | Existing full current copy-vs-in-place master/meter comparisons retain every scenario, mode/shape/count assertion, poisoning and later unmarked recoloured-gather hazard. These are not wire, ABI or single-owner cross-target corpus hashes. Historical research/mutation evidence remains in git and `MUTATIONS.md`. |
| Four inert hashes | Same four shapes × quanta `{1,7,16,128}` × sixteen played-script blocks, including input poison, host padding, census, original production table, ordered dispatch, meter publication and audible-input checks. Before overriding the reference table, the helper validates the production table; reference dispatch is ordered `0..units`, and its actual dispatch count must include every emitted unit each block. Full master and meter equality now owns the no-skip behavior. |

Rewritten test purposes for the verdict: the hostile-width family catches vector/tail disagreement on signed zero, subnormals, infinities and NaN words that a random current-backend traversal cannot assure; the seeded fan-in family reaches chunk carry/reassociation and aliased/repeated/silent source mutation at 2/7/8/9/15/16/17/63/64/65 inputs while retaining the generator's discriminators; the scalar reference callers catch wrong routed-contribution order, live-master continuation and read-before-own-write defects without sharing the production reducer; the fixed analytic cases catch a shared erroneous association; the inert family catches omission of an observed/aliased/delayed input's PCM or meter work across small/ragged quanta, while the source/dead families retain full current copy behavior and lane-marking hazards. No permanent historical reference-output pin was added.

#### Complete retained behavioral-family review

All **89 unit tests** and **five test-support integration tests** remain active. The following are the reviewed homogeneous families and their load-bearing purposes; similar scaffolding was retained where generators or independent defects differ.

`lib.rs` (29):

- Resource transactions: bank/scalar/runtime-metadata overflow rollback, current derived charges and split-table eligible/declined admission.
- Bank attachment: partial padding, empty/oversized rejection and refreshed lowering after attachment; analytic level-major execution and ID-ordered transactional refusal with reusable owners.
- Binding and structure: union/duplicate/overlap/missing/extra/observer/envelope diagnostic precedence; node/level/edge/schedule/spec/bank-lane corruptions; source and plain paths retain common structure checks after binding validation. Public hand-built plans still require these checks.
- Route-fold preflight: eleven installation faults in source and nonsource plans return the original owners without begin/copy/drop/observe, then permit a correct retry.
- Basic ownership/identity: exact independent dual-mono delay, duplicate refusal/ownership return, identity acknowledgements and compile plan identity.
- Alias and observation: alias vs materialized current PCM and precise producer observation window; permanent skip vs unconditional dispatch preserves PCM, trace/order and observers; no-observer plans make zero observer calls.
- Random DAGs: fifty seeded typed graphs reach bind/render and produce nonsilent PCM across effects, sidechains, buses, sends, matrices and PDC. Despite its name, this case alone does not independently prove determinism/bits; the differential/analytic owners do.
- Live control: next-block independent-lane parameters, reversible latency-preserving bypass, inert idle queues and no dropped command within prepared capacity.
- PDC: typed main/sidechain alignment, enabled/bypassed latency at four launch rates and quanta 1/127/128/255/1024; exact delay before ordered fan-in.
- Source lending: only owned claims and exactly the declared quantum are accepted, with short/out-of-bounds refusal.

`program/tests.rs` (14):

- Literal lowering: alias/in-place identity, fan-out refusal and stable fan-in, 64-track ordered nonalias master, delayed staging/boundary retention and taps that do not become physical readers.
- Seeded semantic DAG differential (300 graphs): independent node values, aliases/delays, runtime order, dedication and arena ceilings; malformed schedules/specs refuse lowering.
- Literal runtime schedules: mixed bank kinds and nonmonotonic IDs cross-check the oracle's unit membership and ordering.
- Direct and merged bank windows: independently interpreted physical writes cannot recycle a slot before its last bank reader.
- Seeded bank-window and cohort-chain families (4,000 each): independent semantic/scatter/route-fold interpretation and current no-window/narrow-window controls expose hoisting and merging hazards; all reach/diversity assertions retained.
- Valid route-fold hazards: independent backwards writer/reader analysis discriminates positive admission, extra slot/route readers and intervening master storage reads.

`runtime.rs` (46):

- Response capture: declared native owner mapping, opaque sink/words, missing owner and declared-but-unsupported hook refusal.
- Observations: resident binding order, one lazy planar fallback and accepted-error short circuit; derived observed flag and actual skipped access; exact-count refusal at the full-meter 2^24 boundary.
- Scalar/split pair admission and failure: distinct matrix destination, delay and original source/copy/queue state; pending faders complete before error; first-slot scratch survives pair acceptance/refusal with and without fold; final adjacency requires matching populations and ignores retired runs.
- Delay: cached node witness agrees with actual line; independent pure-shift and VecDeque/sample reference across partitions/reset/lane, including signed zero and NaN.
- D9 reduction: every width/domain/tail; seeded fan-in/group edges/alias and unchanged input words; signed zero, repeated/silent/self/unrelated buffers; fixed left-to-right association counterexamples; fan-in zero and in-place singleton behavior.
- Real folds: actual cohort dispatcher rather than forbidden per-lane callback; independent routed inputs plus scalar reduction; later cohort continues live master in D9 order; graph-specific malformed fold premises refuse before any route/master mutation.
- Resident folds: current staged-scatter differential for both widths, ragged/full tails, hostile coefficients/cohorts, store/accumulate and no source overwrite; broken premises refuse before writes; resident and staged first-contributor signed-zero laws remain distinct paths.
- Source dedication: zero-input fill is skipped only for a bound source; gather-source admission retains all independent kind/input/staging/sidechain refusals.
- Routes: frozen multiply/add order against independent soft-f64 reference, vector/tail and nonassociation discriminator.
- Metered folds: per-stage admission/decline and current withheld-path reference; complete PCM and meter-window equality across width/partial/multicohort/residency-fallback cases.
- Scatter redirects: hostile identity NaN payloads, observer/alias/fader/route admission, current declined-path PCM/meters, retired-route unit identity and exclusion of the metered redirect consumer from split pairing.
- Host output: current arena-reference PCM/meters for eight shapes, strides/padding and retained arena words; output-reader refusal; begin/process/observer/source failure clears both host planes, whereas pre-rejected renders leave words untouched; current host-plane reduction vs arena reduction.
- Source lending: nine shapes retain full/ragged banks, mono/signed-zero, short tails, underrun, delay, observed/direct routes, compensation and later reused gathers; all masters/meters/modes/counts compare to the current copy arm.
- Inert inputs: plain, observed, aliased and delayed shapes retain table/census/publication/dispatch/poison/padding checks with the current full-dispatch owner; metadata covers actual bind-sized tables by ceilings.
- Dead claim: actual later unmarked recoloured gather is demonstrated before comparing current copy/in-place masters/meters and live/dead claim mode/counts for both redirect choices at all four quanta.

RT integration (five with `test-support`): direct/folded prepared render has analytic PCM, real bank dispatch and zero allocations/frees; played-source lending vs copy matches whole masters over 1,000 blocks with mono/signed-zero/underrun and zero allocations/frees; resident vs current nonresident acquisition compares queued controls, observer words, source/state/trace, transpose and mono disengagement/recovery under full/partial/tails and begin/process/observer faults; crossfeed/extra-reader/delayed-send/incompatible-width controls retain current scalar admission/PCM; real resident prepared plans check zero allocations/frees and actual accepted transitions. Existing warmed audit negative controls remain.

#### Actual checks and limits

All Cargo commands used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b` and `--locked`.

- `cargo test -p graph --lib`: 89 PASS after the first tranche. Reference helper completion: `--lib reduction` 9 PASS and `--lib fold` 14 PASS.
- `cargo test -p graph --features test-support`: 89 unit + 5 integration PASS; same command `--release`: 89 + 5 PASS. These full gates cover the final production and scalar-reference changes; the later completion changes only cfg(test) source/inert/dead owners.
- Root additionally reports integrated-checkout test/doc gates: 154 PASS / 1 ignored before the final hash conversion. This supports integration of the production cleanup, not execution of the later rewritten source/inert/dead families; their final targeted runs are listed next.
- Final hash conversion: `cargo test -p graph --lib source` 12 PASS and `--lib claim` 3 PASS, including all four inert shapes and both dead-claim redirect choices.
- `cargo clippy -p graph --all-targets --all-features -- -D warnings`: PASS after each bounded completion, including final hashes. Existing invalid configured paths `math::fast_db::{fast_level_db,fast_gain_from_db}` emit configuration warnings; no code lint failure.
- `cargo fmt --all -- --check`, `git diff --check`: PASS. Existing `check-graph-policy.sh`, `check-realtime-policy.sh` (54 regions / 15 files), `check-lane-policy.sh` and `check-workspace-policy.sh`: PASS.
- `RUSTFLAGS='-C target-feature=+simd128' cargo check -p graph --target wasm32-unknown-unknown`: PASS; `cargo check -p graph --target aarch64-apple-ios` and `aarch64-linux-android`: PASS. Production compilation only; no Wasm/NEON execution or device/link qualification claim.
- `cargo rustc -p graph --release --lib -- --emit=asm`: PASS with repository AVX2/FMA flags; inspected current fold/reduction/resident symbols and copy tail jumps as recorded above. No base assembly or timed comparison, so no speed percentage or instruction-reduction claim.
- Downstream `cargo test -p graph-compiler --all-features --test bypass_resources`: 3 PASS; `--test bank_levels the_sixty_four_track_console_less_one_eq_binds_at_every_width`: 1 PASS. An earlier mismatched bank-level filter selected zero cases and is not counted as a gate.

Deferred existing owners: #1154 owns safe public arena alias/write boundaries; #932 owns hostile hand-built source physical claims; #931/#932 retain structural/memory admission implications. Split layout mirrors remain executable resource-estimate inputs: changing them changes admission/seals. `UnitIdentityWithoutFlags` stays as #943/I6's target-compiled zero-growth packing invariant; it does not account for heap ownership or prove RT behavior. #1074 owns actual retained-heap accounting; #967 owns large-plan scatter redirect asymptotics; #1078 owns 65,537-track host preparation qualification; #952 owns missing W4 resident-fold Wasm execution evidence. No new owner question, benchmark, listening claim or unrelated implementation was introduced.

### Root adversarial review — attempt 1

Root adversarial verdict: PASS. Reviewed all three implementation checkpoints and the complete five-axis/family evidence. The metadata transaction preserves overflow precedence and commits only after all sums succeed; borrowed identifiers preserve ordered structure and attachment diagnostics. The removed fold checks are constructor-proved immutable invariants, while mutable lease/table/master preconditions remain. Current scalar reduction definitions preserve read-before-write and ordered addition without copying an old implementation or allocating temporary samples. Historical source/dead hashes duplicate stronger current full-word comparisons; the inert replacement independently forces every emitted unit through the current executor, retains the original table checks, and asserts actual reference dispatch counts. The two retired layout tests do not defend a current boundary contract. Public resource-estimate mirrors and the existing padding invariant remain because their current admission/packing purposes differ. Product gates, target compilation and final affected test gates support the changes with the limitations recorded above; no unmeasured speed claim is made.

Test value: `every_lane_width_matches_the_scalar_reference_on_hostile_values` rejects vector/tail disagreements on directed NaN words, signed zero, subnormals and infinities that random finite sampling does not assure. `random_fan_in_reductions_match_the_scalar_reference` reaches chunk-edge carry/reassociation and repeated, silent or aliased input corruption across its retained fan-in generator and discriminators. The three existing scalar-reference callers reject wrong route contribution order, live-master continuation or writing a destination before all same-frame inputs are read; the analytic association case independently rejects an association error shared by implementation and sampled references. The plain inert case rejects skipping work that changes poisoned/padded PCM or meter publication; the observed-input/alias case rejects suppressing observation through either binding; the delayed case rejects treating stateful input work as inert across ragged blocks. Their full-dispatch reference reaches these omission defects without pre-change byte pins. The source and dead-claim cases retain their existing copy-arm equality, explicit mode/count checks and demonstrated recoloured-gather hazard. No new test family is added.
