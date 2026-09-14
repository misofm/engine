# Make builtin input HPF and LPF live through prepared filter targets

## Parent, prerequisite and smallest product slice

Delivery child of #804, after dedicated live-EQ child #807. Reuse that child's opaque preparation helper, companion transport, acknowledgement handling, generation/revision checks, frozen queue drain and whole-batch admission. The user authorized Astra XHIGH scope, bounded sequential Luna XHIGH implementation and fresh Astra MEDIUM adversarial review. Root creates matching numbered local/GitHub specs before implementation, commits each coherent tranche, and owns delivery. Maximum five attempts, each receiving one adversarial verdict.

This child makes existing input-stage filters live in browser and headless consoles. It adds no EQ instance, general plugin control framework, variable filter order, arbitrary coefficient API, timeline automation renderer or session-response protocol. It does not depend on #774. The app continues to expose cuts in its one EQ and prepares both builtin cutoffs as zero; this engine capability remains independently supported even though the app hides it.

Source inspected: #804, the D2 ruling, `InputStage`/`SvfSection`, lane input kernels, builtin compiler bank/scalar drains, host-web admission and SDK writer/console in `/tmp/miso-engine-804`. The existing builtin `SvfSection::design` uses its own frozen f64 operation order; do not substitute the EQ designer merely because both filters are TPT SVFs.

## Frozen DSP and semantic contract

Keep builtin IDs **3 `hpf_hz`** and **4 `lpf_hz`**, units Hz, exact +0 disabled, independent Left/Right, Both simultaneous, fixed second-order12dB/oct Butterworth Q=1/sqrt(2). There is no independent enable field or remembered last enabled frequency in the engine: disable is zero, enable supplies a positive cutoff. There is no slope or Q command. #191 higher/variable slopes remain post-launch scope, as decided by #804/#805.

Use the existing `validate_builtin_filter_cutoff` and `builtin_filter_cutoff_maximum_hz` authority, including its exact rate-specific inclusive f32 maxima, minimum10Hz and open Nyquist requirement. Do not copy EQ's20kHz ceiling. All four launch rates are supported; extended research rates do not become host support. Preserve the existing exact-zero/domain policy rather than silently treating malformed disabled values as a new sentinel. Each lane's semantic target pair satisfies `hpf == 0 || lpf == 0 || hpf < lpf`.

The two rows become `BlockTarget`/live-updatable only when host admission and both render paths apply them. Add `BuiltinSmoothingPolicy::Linear64CoefficientUpdates`, emitted as `linear64CoefficientUpdates`, so metadata states a fixed64-update coefficient ramp rather than promising arbitrary-N Hz interpolation. For these new live rows use the existing truthful `KeepTargetResetCurrent` reset policy. Other builtin descriptors and policies keep their meanings. Add IDs3/4 to existing session-automation target syntax/gates as required by the D2 live/automatable rule, but preserve the explicit fact that the session automation table has no render feed. Do not claim this child implements that feed.

## One appended command kind, including atomic pair edits

Append `COMMAND_INPUT_FILTERS` with generated name `inputFilters`; baseline's next free value is12, and root verifies that value when this child starts. No reserved holes or renumbering. Keep the48-byte command/report ABI. Fields common to this kind: `rack=255`, `effect_index=0`, `channel=0/1/2`, `smoothing_samples=0` meaning the declared fixed64-update policy, and all reserved bytes zero.

| parameter_id | values | meaning |
|---|---|---|
|3|`[hpf_hz,0,0,0]`|Set HPF, retaining each selected lane's current accepted LPF target.|
|4|`[lpf_hz,0,0,0]`|Set LPF, retaining each selected lane's current accepted HPF target.|
|0|`[hpf_hz,lpf_hz,0,0]`|Set the final pair atomically on the selected lane(s). Zero here is a command selector, not a new parameter ID.|

Other selectors or malformed unused fields are refused. Use existing `domain` for cutoff/order violations, `malformed` for wrong-shaped records, and ordinary unknown-track/wrong-state/backpressure results. Refusals retain original wire indexes.

SDK `TrackEdits` adds `hpfHz(value, {channel?})`, `lpfHz(value, {channel?})` and `inputFilters({hpfHz,lpfHz}, {channel?})`. The pair method produces **one LaneEdit/one original wire command**; the writer can never split its two values into separate submissions. Accept explicit `Hz` values through the existing generated unit convention where #147 exposes structured builtin values; never add handwritten aliases. Methods expose channel selection without an arbitrary smoothing option. A client supplying a nonzero raw smoothing window receives a refusal.

Each original command is a valid semantic transition: validate the one-sided result against the control shadow's opposite value, or validate the complete supplied pair at once. Apply valid commands in wire order to a transaction candidate, validate all later commands too, and commit only after whole-batch admission. Example: `(HPF100,LPF1000)` -> `(HPF2000,LPF5000)` succeeds as one pair command; an HPF2000 one-sided command against LPF1000 fails, even if a later command would raise LPF. The pair is the supported operation when independently valid old/new pairs lack a valid intermediate. Do not let last-wins coalescing conceal an invalid original command.

## Small builtin-owned preparation and queue target

Add a small `crates/builtins/src/filter_control.rs`. It owns typed pair validation, final target design and prepared-word safety checking, all using the current builtin design and response coefficient authority. Factor preparation's existing checks into shared functions instead of designing all builtin stages to prepare two filters. No new DSP crate or dependency from builtins onto parametric-eq.

The final per-track shadow is exactly `[left_hpf,left_lpf,right_hpf,right_lpf]` plus dirty flags and revision, seeded from the accepted normalized track builtins. One committed and one candidate4-f32 array suffice; use preallocated per-track storage in the existing host control owner, only when its input console queue exists. Do not recover cutoff from an inverse of current coefficients and do not seed it from response curves or SDK defaults. Shadow lifetime/generation follows the live host, with the same ack/rollback rule as EQ. A valid zero-to-zero edit may update nothing, but refusal never changes the pair or revision.

Use section-sized targets to keep the EQ child's shared transport and count bound unchanged. The companion target address is `(track, rack=255, effect_index=0)`, with `slot=0` HPF or `slot=1` LPF and existing Left/Right/Both selector. Its12 payload words are:

- words0/1: normalized f32 bits for that lane's **final full target pair** HPF/LPF;
- words2..5: zero reserved words;
- words6..11: `c1,a2,a3,m0,m1,m2` for the addressed section, rounded by `SvfSection::design`.

The target's other cutoff is semantic association data, not an instruction to retarget the other section. Emit only sections touched by the batch, so changing HPF never restarts an existing LPF ramp. Coalesce repeated edits to each final section/lane. Both is one record only when both lanes are addressed and the **whole final pair and target words** agree; otherwise use Left/Right. Complete pair changes install both section targets before any sample is processed.

At most4 final targets exist per builtin track. The shared cap stays `2 * MAXIMUM_COMMAND_RECORDS = 512` per submission: with one command on a track, even a paired Both command emits at most2 targets because the whole final pairs agree; with two or more commands, the per-track maximum4 is <=2 times its command count. Summing over owners preserves the EQ child's bound. This is not a track limit. Count actual coalesced targets against input queues and shared staging; a depth1 input queue can legitimately refuse a two-target paired change. Do not acknowledge the first half to make it fit.

Decode the validated companion into a builtin-owned fixed `Copy` type, for example `PreparedInputFilterTarget { lanes: BuiltinLaneSelector, section:u32, pair:[f32;2], coefficients:[f32;6] }`, appended as `TrackInputRecord::PreparedFilter`. No owned heap payload, pointer, separately recycled slab or new queue. Require this record enum <=64 bytes and account for its actual size; every allocated input queue grows by its `(depth+1)` slot count. The EQ child's shared decoded-command union should already accommodate this size. Non-input queues do not grow again for this child.

`InputBuiltins::apply_prepared_filter(target)` and `BuiltinInputBank::apply_prepared_filter(lane,target)` share the width-generic InputStage setter. Check shape before writing, then update only the addressed current-to-target ramp state. No `tan`, cutoff inversion, descriptor parsing or allocation is reachable there.

## Reuse the common helper and whole-batch bridge

Extend the **existing live-EQ helper owner**, not a second helper. It has one compiled module/no-boot main-realm Wasm instance, one owner cache, one outstanding coefficient-prepared batch, one generation/timeout/disposal path and one companion submission. Its owner key already contains track/rack/effect: `(track,255,0)` distinguishes builtin input from EQ racks0..2. A mixed EQ+builtin+fader batch is prepared completely, transmitted once, admitted once and committed once. The busy bound now covers any batch containing EQ or builtin prepared controls; do not create independent EQ and builtin in-flight lanes that race shared admission.

Reuse the companion frame and12-word target layout; `rack=255` selects the builtin decoder, while EQ targets keep their exact layout/semantics. Reject every unsupported family/address. A new small builtin preparation export is acceptable because the mathematical owner differs: e.g. `miso_engine_web_v1_input_filters_prepare`, with explicit launch rate, the4-f32 seed, and <=256 fixed edits `(parameter_id:u32, channel:u32, value0:f32, value1:f32)`. It validates/updates the candidate in wire order and returns the4-f32 final config and <=4 section targets. Use the already owned/prewarmed target workspace and output capacity, with checked fixed input buffers; there is no second stateful transaction object in Wasm and no extra worker/host boot. A small builtin adapter in the same JS helper dispatches this Rust function; it implements no cutoff, pair or coefficient math in JS.

The new read-only addressed builtin config-copy export copies current generation, revision, sample rate and4 target values from Rust. Use the EQ helper's existing bounded seed-request/response machinery and fixed scratch ownership; publish the correct per-family value count rather than pretending the builtin has60 values. The existing EQ export/layout remains valid. Main/helper lazy seed, unchanged-output provenance and ack-owned candidate commit are identical to EQ. Headless calls the same stateless builtin export in its existing instance between render calls. Do not make browser receiveCommand or Rust worklet admission call it.

Extend the existing host admission transaction: decode all semantic commands; stage EQ and builtin candidates plus ordinary/solo lowering; match all prepared payloads to final candidates/generation/revision; validate domain/shape/numerical safety; count every destination's capacity; publish records; then commit **all** shadows/revisions and issue the unchanged original-command report. A refused mixed batch changes nothing anywhere. Each target pair header must equal the final shadow for its channel and each required section/channel must be present exactly once. The internal trusted-helper provenance contract from the EQ child applies unchanged: numerical checks do not prove arbitrary supplied words implement the declared cutoff. Public APIs accept no caller-authored sidecar. Response copies actual applied words.

The builtin validator additionally requires exact fixed HPF mix `(1,-BUTTERWORTH_K,-1)`, LPF mix `(0,0,1)`, or the exact six-word identity when that section's target cutoff is0. For an enabled target require finite normal-or-zero words, `0<c1<=1`, `a2>0`, `0<=a3<=1`, and the spectral norm of `A=[[1-2*c1,-2*a2],[2*a2,1-2*a3]]` <= `1+2^-22`, the existing rounded-SVF tolerance. Evaluate the bounded f64 two-by-two norm formula in this builtin owner (sqrt is allowed); no transcendental designer is called. This criterion reuses the EQ mathematical authority without creating an effect-crate dependency. Builtin near-Nyquist endpoints require the stated inclusive `c1<=1`, rather than EQ's strict `c1<1`. If an existing legal fixture fails this criterion, stop and record the concrete counterexample for scope review; do not narrow the advertised domain or relax the gate silently. Test all rate-specific maxima and immediate successors. Stable semantic forgeries remain outside the opaque internal ABI's provenance precondition, exactly as for EQ.

## Runtime ramp, elision and mono rules

The filter's law is the EQ SVF law: at retarget, `step=(target-current)*2^-6`; sample A processes with current words, then the six words advance; after64 updates snap exactly, and sample A+64 uses the exact target. Authority: `crates/lane/src/kernels.rs::svf_block_ramped`. The existing trim ramp is different: it advances before applying its frame, and its law must remain unchanged. Retarget starts from current coefficients, not an old target; no-op settled targets stay settled.

Keep current coefficients solely in `InputStage.coef.section`. Add bounded per-section target/step and countdown state for2sections*2channels*bank width, a filter-ramping flag, and the initial prepared target words needed for an explicit full reset. Store semantic target cutoffs/enable facts only where needed for honest response/initial reset; the host shadow is admission authority. No per-track heap in a callback; padding lanes remain identity with zero countdown.

Add one width-generic combined filter/trim ramp body and its mono specialization in `crates/lane/src/kernels/builtins.rs`, reusing `svf_step` and existing sanitization/mix operation order. It advances trim with its existing law and filter words with the SVF law; scalar, Simd4 and Simd8 instantiate the same arithmetic. Leave settled-filter dispatch on current stationary/elided kernels. For trim-only ramps, add the narrow all-identity plan dispatch below; retain the existing unelided trim-ramp arithmetic as its fallback. Process a prefix of at most64frames through the new filter-ramp body, then dispatch the remaining suffix through the appropriate existing settled-filter body. Per-lane countdown/masks handle earlier completions inside that prefix. Combine sanitized counts/nonfinite masks across segments and perform the existing whole-block recovery once; a failure in a prefix must not silently change recovery from whole-block to prefix-only. No unbounded segment search or coefficient design.

### Required all-disabled bypass during live trim/polarity ramps

This is part of the user's disabled-filter product contract, not a deferred optimization. The app prepares HPF/LPF0 on both channels of every track; those filters must execute **zero SVF recurrences during trim or polarity ramps as well as during steady trim**, once the sections are settled at their proven identity. Retain the input stage's sanitization, actual trim/polarity work and signed-zero normalization.

The smallest implementation is two plan dispatch wrappers in `crates/lane/src/kernels/builtins.rs`: `input_chain_ramp_block_elided` and `input_chain_ramp_block_mono_elided`. They receive the existing `InputChainPlan`. The dual wrapper selects a dedicated identity-trim-ramp body when `plan.elided == [[true,true],[true,true]]`; otherwise it calls the existing `input_chain_ramp_block` unchanged. The mono wrapper selects its channel0 identity body when `plan.elided[0] == [true,true]` under the existing symmetric-plan precondition; otherwise it calls the existing mono ramp body unchanged. This adds one pre-loop choice, no per-sample plan test, no general mixed-plan rewrite and no new render allocation. `InputStage::process/process_mono` call these wrappers in the trim-only arm, including the trim-ramping suffix after a filter-disable transition completes.

The identity-trim body is exactly the existing ramp body's ramp/sanitization/output accounting with its two-section loop replaced by **one `v = v.add(+0.0)` immediately after trim**. Per frame: decrement the existing trim countdown; select target or `current+step` by the existing rule; retain the new current; sanitize the original input with the same ordered magnitude comparison and counting operation; multiply sanitized input by that trim; add+0 once; perform the existing nonfinite output mask accumulation; store. At block end publish only the trim ramp current/countdown and report. There are no SVF coefficient reads, integrator loads/stores, calls to `svf_step`, or filter coefficient updates in this body. The mono body advances only channel0 and duplicates report fields exactly as the current mono path; InputStage's existing trim-ramp mirroring remains responsible for channel1.

The +0 addition is mandatory, including when trim passes through zero or input is -0: `section_is_identity` and `identity_chain_block` already prove that each disabled identity section maps `v` to `v+0`, and two adjacent identities collapse to one such addition. A plain pass-through/multiply would change signed-zero output. Sanitization occurs before trim and output checking remains after the addition, so NaN/Inf/magnitude policy and whole-block recovery remain unchanged. The plan still requires exact identity coefficient bits and +0 integrators on every bank lane; it is never inferred from a UI flag or cutoff alone. An in-flight filter ramp forces the plan false until its exact disabled snap/clear, so this fast path cannot suppress enable or truncate a disable transition.

Existing mixed/nonidentity bank behavior is left intact: if some lane's active filter prevents the all-lanes identity plan, that bank uses the correct fallback. The app's all-tracks-disabled builtins contract satisfies the all-identity predicate even with arbitrary independent live trim/polarity ramps. For that case, filter DSP is absent; actual trim, sanitization, one identity-normalization add, branch/flags and preallocated storage remain. A fully disabled input does not mean bypassing trim or polarity themselves.

At the exact final disabled snap, clear that section's integrators for the addressed lane to +0 **after** the last pre-target sample and before the first exact-identity sample. This removes hidden old filter memory only once its output mix has reached identity. Do not clear state on enable, ordinary sweep or at the start of disable. Mid-ramp retarget to enabled retains the current state and cancels the pending disabled endpoint. No epsilon/time-based premature clearing. Reuse existing denormal flushing/nonfinite recovery and signed-zero identity rules.

Implement one `refresh_filter_plan` authority: start from `input_chain_plan(coef,state)`, then force `elided[channel][section]=false` whenever any lane of that section has an in-flight filter ramp. Call it at retarget, ramp completion/disabled state clear, reset, explicit state restoration/evidence mutation and mono state restoration where appropriate. Merely recomputing from **current identity** words at enable admission is wrong: current can still be identity while its target is active. The new ramp body must run even when the old plan said every section was elidable. At completion, recompute from exact settled words/state so permitted identity elision returns. Keep conservative non-elision if any populated or padding lane's identity predicate declines; never change another lane's arithmetic to optimize a disabled lane.

Extend the cached channel-symmetry read surface and debug oracle to current/target/step/countdown for every live filter as well as existing trim words. Refresh it after filter retarget, each ramping process block, mirroring and reset. `supports_mono_collapse` must reflect the current symmetric elision plan, not a prepare-only claim. `channels_agree` includes the new ramp words/countdowns, so a stale right ramp cannot re-earn collapse.

Use the input stage's **existing mirror rule**, which avoids new deferred staging machinery: `BuiltinBankProcessor::begin_block` freezes each queue's available-entry count using the EQ child's SPSC API, drains that bounded prefix, folds `LiveConsoleRecord`, and applies filter targets before the collapse witness is read, as it does trim. The scalar ConsoleInputProcessor uses the same frozen drain. `PreparedFilter` Left/Right clears LIVE, Both preserves it. A collapsed process advances channel0 filter words/ramp and mirrors the complete current/target/step/countdown and relevant target semantics onto channel1 at block end, as it already mirrors trim. Only integrators are frozen. Therefore `InputStage::desymmetrize` continues to copy **integrators only**, never freshly admitted filter targets; refresh the plan after the copy. Do not copy EQ's whole-section disengage behavior into this owner. The existing one-way LIVE latch remains unchanged.

## Reset, response, tails and accounting

`InputBuiltins::reset()` / `BuiltinInputBank::reset()` keep accepted filter targets, snap current coefficients to those targets, clear integrators and countdowns, recompute plan and symmetry; they never redesign. A new explicit filter reset-to-prepared helper restores cached initial words/cutoffs without design and is used by `BuiltinChain::reset(FullToPrepared)` for the filters. It does not widen this issue to repair unrelated trim/fader reset semantics. A host invoking an explicit full reset also resets its filter shadow/revision coherently; normal session replacement invalidates old generations and seeds. Do not resurrect an old admitted target after reset. Existing integrator-state evidence remains in its original8-word order; use separate bounded ramp-state evidence accessors. This child does not introduce a general serialized builtin state API.

Current `InputStage::copy_response_snapshot_lane` calls `lane_track`, which reads **current** coefficients and infers enabled from its output mix. That is no longer the right source for the accepted live **target** response. Copy retained target coefficients and target enable facts instead, with response IDs1HPF/2LPF and existing seven-word order `c1,a2,a3,k,m0,m1,m2`; `k` is the fixed Butterworth constant or0 at a disabled target. Do not put the six-word transport directly into the seven-word response layout. Requested/stopped response preparation keeps the existing owner formula. Ack A starts a transition; after the admitting block a fresh capturedSample>A snapshot reports the same accepted target, and PCM is at that target after64updates. A target curve does not describe the instantaneous in-ramp response. There is no builtin effect-wide bypass; cutoff0 supplies identity.

The compiled graph currently derives builtin tail from its initial enabled sections (`builtins-compiler::expected_tails`, preparation and graph-compiler lowering). A console-controlled input can activate an IIR after an initially disabled prepare, so it must conservatively declare **Infinite** tail for that live-capable plan. Update the expected-tail/seal derivation to include accepted input control attachment. A no-console disabled input still declares FiniteZero; no render-time structural tail mutation is allowed. This is a truthful bound, not evidence that a disabled filter is running. Any direct runtime tail query must not say zero while a filter ramp or active state remains.

Account separately for added current-independent target/step/initial words, per-lane countdowns and symmetry state, input queue slot growth, host4-value shadows and shared workspace increment. Existing6-word current SVFs and8integrators per track are already retained. The fully disabled, settled identity-elidable input does **zero SVF recurrence work with either steady or ramping trim/polarity**, using the required all-identity ramp dispatch above, while retaining the input stage, sanitization, trim, signed-zero normalization, flags, queue capacity and coefficient/state storage. Do not say zero total resources or zero input processing. Active enable/disable transitions still process their bounded ramps until the exact disabled endpoint. Mixed banks follow their all-lanes elision rule. Document the floor by actually active/elided sections and the bounded ramp path, with no unmeasured performance promise.

## Three bounded sequential Luna tranches

Root owns issue synchronization/checkpoints. No new tranche starts before the previous compiling, focused-green tranche is committed. Only the live builtin feature issue is active implementation WIP; no other-effect edits.

1. **Builtin owner, ramp and identity behavior.** Paths: `crates/builtins/src/{lib,filter_control,filter_response}.rs` (`filter_control` new); `crates/lane/src/kernels/builtins.rs`, plus `crates/lane/src/kernels.rs` only if extracting a genuinely shared coefficient-advance primitive; `crates/lane/tests/input_chain_elision.rs`; `crates/builtins/tests/{input_liveness,input_liveness_mono,mono_collapse,filter_response,contract}.rs` plus one focused `filter_liveness.rs`. Implement pure pair design/validation and fixed target, new filter-ramp body, the required all-identity trim-ramp dispatch/body, plan invalidation, mono mirroring/symmetry, target response and reset. Keep metadata prepared-only until host closure. Checkpoint: focused builtin/lane tests, independent response/partition comparisons, native+Wasm checks. New runtime methods can be tested with off-thread prepared targets without pretending the public host already admits them.
2. **Input queue and shared host admission.** Paths: `crates/builtins-compiler/src/lib.rs`, `tests/{input_drain,allocation_tracker,builtin_automation_targets}.rs`; only concretely needed tail checks in `crates/graph-compiler/src/compile.rs`/tests; `hosts/host-web/src/{lib,ffi,tests,control_targets}.rs` as created by the EQ child; `crates/host-core/src/control_preparation.rs` only its builtin facade; actual resource-accounting paths. Implement the new TrackInputRecord arm and bounded drains,4-value owner shadow, command kind/pair validation, same helper FFI workspace, seed export and mixed-batch transaction; tail capability/seals. Checkpoint: focused compiler/host tests, empty/full/invalid/stale mixed-batch proofs, allocation and Wasm gates. No second queue or admission manager.
3. **SDK/helper, truthful metadata and shipped parity.** Paths: existing shared prepared-control JS helper and `hosts/host-web/web/miso-engine-v1-audio-worklet{,-host}.js`, host `.d.ts`; `sdk/src/core/{console,boundary}.ts`, precise existing semantic builtin binding path from #147 if present; `crates/builtins/src/lib.rs` descriptor flip; `crates/session/src/validate.rs` syntax allowlist; `tools/parameter-metadata/src/{lib,abi_layout}.rs`, schema/validator vocabulary and generators; generated SDK/catalog/ABI/assets; focused SDK console/headless/browser/response tests. Add the builtin family adapter to the same helper lifecycle, preserve atomic pair LaneEdit/coalescing, publish the smoothing/units/live metadata, update automation-syntax documentation honestly, and test real browser plus headless through the shipped facade. Root chooses proportional complete gates once; one fresh Astra MEDIUM verdict reviews the coherent attempt.

Root documentation/evidence paths: numbered child/#804 specs, D2 liveness ruling, relevant `effect-floor-accounting.md` inventory, and existing session/control docs. Update source comments that currently rely on prepared-only cutoffs. Do not repin whole historical corpora or create a benchmark framework to close this feature.

## Required acceptance evidence

- Pure pair preparation and live DSP at44.1/48/88.2/96kHz: HPF-only, LPF-only, both, disabled; minimum and exact rate-specific maximum plus successor rejection; finite results and numerical validator agreement with the preserved builtin formula.
- Identity-disabled -> enable -> sweep -> disable; initial sampleA/current and target sampleA+64; midpoint retarget; no unrelated-ramp restart; awkward quantum partitions including1,17,63,64,65 and128; scalar/Simd4/Simd8/tails. Concurrent trim and filter ramps preserve each law and sanitization/recovery counts. With all filters settled disabled, scalar/Simd4/Simd8/tails and mono/dual trim/polarity ramps take the identity-trim body and match the old unelided ramp reference bit-for-bit for PCM, current/target/step/countdown, sanitization and recovery. Include negative samples, +/-0, polarity crossings, NaN/Inf and output-limit recovery. Assert unchanged +0 filter integrators and absence of filter recurrence work using test-only dispatch instrumentation at the actual identity-body entry plus a direct source/compiled-path check; PCM equality alone cannot prove elision. A mutation routing the wrapper back to the old unelided body must fail the mechanism gate. Cover return to this path in the suffix of a disable-completion block while trim is still ramping.
- Correct plan before admission, while identity-to-active ramp starts, at each completion and after reset; final disable reaches exact identity and clears only the addressed state at the defined boundary; signed-zero behavior agrees with the existing recurrence/elision authority. A last/second section regression makes LPF audibly/measurably active.
- Actual compiled scalar and bank input queues exercise Left/Right/Both. Both while collapsed matches an always-dual oracle; first asymmetric filter edit during collapse preserves the newly admitted target and restores only frozen integrators. Filter target/step/countdown mismatches prevent collapse, without altering the LIVE latch.
- Atomic paired valid-to-valid crossing succeeds; one-sided invalid crossing rejects. Both must validate both lane pairs, including unequal opposite cutoffs. Invalid late values, missing/duplicate/overlapping/corrupt targets, stale generation/revision and a full unrelated queue refuse a mixed EQ+builtin+fader batch with unchanged queues and all shadows. Original command counts/indexes remain truthful.
- Frozen queue drains cannot chase new producer writes; target count remains<=2N/512 and never imposes a track cap; input queue/staging/host/workspace resources are charged from actual types. A paired command is never split into two independently acked values, including writer backpressure/coalescing.
- Real browser helper never designs on AudioWorklet thread and shares exactly one preparation instance/owner with EQ. Headless uses the same Rust preparer between renders. Designer-call instrumentation plus callback call-graph inspection and allocation/free counters cover prepared admission, render, reset and mono transitions.
- Fresh copied target response carries actual accepted words/enabled facts, correct seven-word layout and HPF/LPF IDs; post-ramp PCM agrees with the existing independent impulse/response authority. Before the admitting block, old captures are not mislabeled fresh. Unsupported arbitrary raw coefficient inputs remain outside the documented trusted helper contract.
- Both reset modes are coherent with their shadows; live-capable initially disabled compiled input has Infinite tail, no-console disabled input remains FiniteZero; settled disabled identity invokes no SVF recurrence during steady trim or live trim/polarity ramps while retained memory and actual trim work remain honestly reported.
- Metadata, commands, structured units, generated SDK types/assets and actual browser/headless capability agree. All target/build/ABI/package gates proportional to touched paths pass. No benchmark is required; if one is run, use the repository's single frozen descriptive invocation policy.

## Precise amendments to the live-EQ brief

These are narrow bridge/wording clarifications, not a prerequisite framework expansion:

1. **Already corrected in the #807 local spec at root request:** replace its inaccurate update-before wording with the source-authoritative process-current-then-advance rule from `svf_block_ramped`, target used at A+64. Trim remains advance-before-use.
2. Treat the helper's lifecycle/transport owner as a shared prepared-control helper, with an EQ family adapter today and builtin input adapter in this child. A neutral internal/file name such as `prepared-control.js` avoids an eventual misleading `prepared-eq-control.js`; this does not change public semantic APIs. Do not build the builtin adapter before its child.
3. The companion address decoding may add builtin `(rack255,effect_index0)` here; EQ still rejects it until this capability ships. The12-word payload and512 target frame cap need no growth. Keep family-owned payload validation; do not reinterpret builtin words with the EQ decoder.
4. Config-copy/preparation adapters may return different exact value counts (EQ60, builtin4) within the existing bounded helper/workspace; add the small builtin export/typed result when needed, preserving the EQ ABI. Reuse one cache/in-flight/ack/disposal implementation. No new generic effect registry, worker owner or scheduler is required.

## Decision and evidence record

Astra XHIGH scope: ready for root's numbered issue brief. Implementation attempt1 and Astra MEDIUM verdict pending. Existing TPT equations, numerical corpus and primary citations remain the DSP authority; this child adds coefficient-ramp scheduling, owner admission and associated response truthfulness. Record observed listening only; do not invent listening evidence. Root records actual checkpoints, focused gates, reviewed verdict and remote issue synchronization before claiming delivery.


### Execution start, 2026-09-14

Root accepts the frozen Astra XHIGH scope and its three bounded sequential Luna
tranches. Attempt1 starts from accepted EQ checkpoint1b81f56a in isolated
`codex/808-live-input-filters`; #807 is PASS/closed and PR#811 is awaiting required
CI/merge. Its implementation is frozen, so #808 is the sole active feature WIP.
Merge the delivered EQ main lineage before final builtin qualification/PR.

Boundary audit found all414 numbered local specs represented among518 GitHub issues;
#804/#808/#809 remain open and #805/#807 are closed. #808's number/title match.
The current command vocabulary ends at polarityInvert11: inputFilters12 is free.
Start only tranche1 builtin preparation/ramp/identity behavior. Keep public metadata
prepared-only until queue/host/SDK closure. Reuse existing fixtures and counters,
run no benchmark, and checkpoint focused-green work before adding another tranche.


### Tranche1 initial DSP checkpoint — implementation continues

Luna implemented the pure builtin pair preparer/validator, fixed64 coefficient-ramp
state and kernels, disabled endpoint clearing, plan/symmetry/reset integration,
target response snapshots, and all-identity trim-ramp bodies. Existing builtin tests,
lane input-chain elision tests and the builtin Wasm check pass; four focused new
fixtures cover launch-rate preparation, first-sample/target response, trim finiteness
and symmetric bank mono/dual PCM. Public metadata remains prepared-only.

Root found two concrete gaps before queue wiring: bank target application currently
updates every SIMD member instead of taking one lane address, and target validation
must associate exact identity with that section's zero cutoff. Correct these in the
same bounded tranche before proceeding; preserve padding and unrelated ramps.
Representative endpoint/partition, asymmetric lane, disable/reset and identity-body
mechanism checks also remain before the tranche is complete. This is a buildable
recovery checkpoint, not tranche completion or a whole-issue adversarial verdict.
No benchmark or new framework was introduced; #808 remains attempt1/open.


### Tranche1 completion checkpoint

The bank setter now addresses one populated lane and preserves neighboring and padding
ramps. Target validation requires exact identity iff the addressed cutoff is +0,
rejects -0 and inconsistent pairs, and retains the fixed numerical safety bound.
Focused gates cover exact endpoints/partitioning, retarget/disable/reset, launch-rate
maximum successors, bank isolation and the actual identity trim-body dispatch.
Builtin and lane tests, strict relevant clippy and Wasm checks pass. No timing run
or new evidence framework was added. Public metadata remains prepared-only pending
host/SDK closure; this completes DSP tranche1, not issue #808.


### Tranche2 queue/compiler checkpoint

PreparedFilter now travels through the existing scalar/bank input queues, whose
drains freeze available-at-entry counts. Its 40-byte record remains within the
64-byte bound; actual slot accounting and the existing mutation transcript reflect
the growth. A first asymmetric filter target reaches the real bank collapse/dual
comparison without being overwritten during disengage. Scalar Both preserves the
witness. Live-console inputs declare Infinite tail; plain disabled inputs remain
FiniteZero, and direct runtime queries account for in-flight ramps.

PASS: nine builtin filter fixtures, six input-drain fixtures, actual bank/scalar
queue checks, existing 10,000-case compiler mutation transcript and allocation
tracker checks, strict relevant clippy, native/Wasm compiler checks. No new queue,
scheduler, benchmark or test framework. Shared host preparation/admission and SDK
closure remain; #808 remains attempt1/open.


### Tranche2 stateless preparation checkpoint

The shared host-core facade now validates four-value builtin seeds and every
original edit in order, supports atomic pairs, emits only touched final sections
and coalesces Both only for identical whole pairs/words. Admission can reuse its
semantic-only edit operation without invoking the designer. The existing helper
workspace grows its request buffer to4144bytes and retains the944-byte result
buffer. Only input_filters_prepare is added; lifecycle, pointers and diagnostics
reuse the existing exports. The builtin result uses four values and the frozen
12-word target layout; refusal preserves the previous valid result.

PASS: four focused host-core checks, three host-web workspace checks, native/Wasm
checks and strict relevant library clippy. Root maps unsupported builtin selectors
to malformed as required by the command-shape contract. Public metadata remains
prepared-only. Next: committed/candidate shadows and shared mixed-batch admission,
then the existing helper/SDK integration. No release or whole-issue PASS yet.

The private builtin unit designer counter records calls at SvfSection::design: real
off-thread pair preparation is positive, while scalar/bank target application,
rendering, mono disengagement and cached reset paths record zero. Its focused gate
passes. This adds no production API or instrumentation feature; actual host batch
allocation/refusal gates and the shipped callback call graph remain for closure.


### Tranche2 host admission checkpoint

Command12 and the builtin `(track,255,0)` owner now use the existing shared
transaction. Four-value committed/candidate shadows are seeded from normalized
track builtins and charged in the host report. Every original transition is
validated without design; final targets must match generation/revision, complete
pair, touched sections and numerical safety before all destination capacities are
checked. Publication precedes all shadow commits and the original-count/sample ACK.
The new input_filters_config_copy export writes48bytes into the existing272-byte
config scratch and reuses its existing pointer. No additional workspace lifecycle.

PASS: host-web feature tests104passed/two existing ignored, including mixed
EQ+builtin+fader success/refusal, builtin late-invalid/stale/unrelated-full-queue
rollback, actual zero-allocation admission/render and actual shadow accounting.
Wasm check passed. Root fixed two iterator-style clippy findings; strict host-web
lib/tests clippy and six focused prepared-owner tests pass afterward.

Public metadata, shared helper adapter, SDK methods and shipped-artifact parity
remain. Final closure must also exercise malformed builtin companion coverage and
the generated/shipped command vocabulary. This is not a whole-issue PASS or release.
