# Add prepared dedicated HPF and LPF sections to parametric EQ

## Parent and authorization

Delivery child of #804. Astra XHIGH scope approved for implementation; the user's explicit workflow is bounded Luna XHIGH implementation and Astra MEDIUM adversarial verification. Root records the GitHub issue number and matching local spec filename/index before implementation. At most five coherent implementation attempts, each receiving one adversarial verdict. This spec owns one independently usable capability: prepared session/native/offline EQ cut sections. The next child owns live off-render prepared updates, and the app release waits for that child. No prepared-only control is advertised as live.

## Product contract

A single `miso.parametric-eq` instance renders dedicated HPF -> original four general bands -> dedicated LPF. Both dedicated cuts default disabled. They consume neither an existing band nor another effect instance. Existing band parameter IDs, descriptor order and meaning remain stable. Existing general bands continue to support every existing kind. Effect-wide bypass covers all six sections with unchanged zero latency.

Adopt second-order **12 dB/oct** TPT SVF cut filters in this release, with independent Q. This explicitly resolves the #804/#191 slope decision for the product slice: variable 24/36/48 dB/oct and first-order variants are deferred to separate post-launch scope. No selectable slope parameter or inert UI slope exists. Reuse the current EQ HPF/LPF design equations and numerical authority, already documented in `crates/parametric-eq/src/lib.rs` and `docs/rulings/`; no new filter algorithm.

## Frozen metadata and session schema

Append the following six parameters after the existing 24 rows. The existing `name` field is the stable machine key; the separate authoritative unit/domain metadata provides semantics. There is no renamed alias or additional competing key field.

| Numeric ID | name | domain | unit/mapping | minimum | maximum | default |
|---|---|---|---|---|---|---|
|65|hpf-enabled|boolean|linear/stepped|none|none|false|
|66|hpf-frequency|continuous|Hz/logarithmic|10|20000|80|
|67|hpf-q|continuous|ratio/logarithmic|0.1|18|1/sqrt(2)|
|81|lpf-enabled|boolean|linear/stepped|none|none|false|
|82|lpf-frequency|continuous|Hz/logarithmic|10|20000|18000|
|83|lpf-q|continuous|ratio/logarithmic|0.1|18|1/sqrt(2)|

All six use PerLane channel policy, readable=true, automation rate=None, smoothing=None/0, automatable=false, liveUpdatable=false in this child. The live successor changes capability only when both running hosts can apply every admitted edit. Values use the existing lattice rules associated with their units/domains. All launch sample rates 44.1/48/88.2/96kHz use the same range 10..20000 Hz, below each Nyquist. Dedicated EQ cutoffs may overlap; no cross-cutoff ordering restriction is invented here. Builtin live control's separate hpf<lpf invariant remains in its child.

Session sparse values preserve default behavior: old sessions with no cut parameters render with both disabled. New explicit values round-trip through the existing typed session parameter representation. Existing SDK prepared `effect()` objects acquire these rows through ordinary generated metadata; no handwritten parameter map.

## Implementation decisions

1. Keep the count of general bands 4 distinct from physical cascade sections 6. Original band parameter/descriptors index 0..3 remains stable; their actual DSP cascade order is 1..4, preceded by dedicated HPF at 0 and followed by LPF at 5. Use explicit mappings where descriptor order differs from DSP order; do not force new cuts into the old six-fields-per-band arithmetic.
2. Each cut is the existing fixed `EqBandKind::HighPass`/`LowPass` design, no kind/gain/shelf parameter exposed. Disabled uses the exact existing identity words. Reuse existing f64 preparation and rounded-f32 coefficient authority, NaN/domain rejection, denormal handling, reset, state, and effect-wide bypass semantics.
3. The stationary renderer must support 6 sections safely. Today backend depth-4 dispatch plus integer `length/DEPTH` assumes divisibility and can drop tail sections in release. Adopt an effective depth 2 for six-section mono/dual interleave selection, padding and dispatch, or use an explicitly reviewed correct remainder implementation. Tests must make the last LPF audible under every backend. Identity elision preserves established sign-of-zero/state rules; disabled cuts must leave original-band rendered bits unchanged.
4. Public response descriptor IDs 1..4 remain the original bands. Append response section 5 `HPF` and 6 `LPF`. Public output order remains ascending IDs 1..6 even though DSP order is 5,1,2,3,4,6. Both requested-configuration and retained-coefficient response snapshots use a single explicit mapping. The response descriptor validator requires strictly increasing IDs.
5. State payload is 6 × 19 = 114 32-bit words = 456 bytes per channel, plus the existing 8-byte common header. Keep prelaunch layout tag 1 and reject old 304-byte channel payloads explicitly by length/header shape. No reinterpretation or fabricated migration. Recount prepared/runtime state and memory ceilings as well as serialized state. Scratch remains 0; latency 0; tail infinite.
6. Recount the EQ's class-A floor by active section count and actual backend cascade padding. This is accounting, not a new optimization loop. No projected or measured performance claim without evidence.
7. This child does not modify the live `EffectControlRecord` path. Current `automate()` designs coefficients inside process for old numeric bands; no new cut row enters that path. The off-render live successor must address it before claiming live cuts; copying this existing function is not evidence of off-render design.

## Bounded implementation tranches

Root owns commits, GitHub synchronization, issue/spec evidence, branch changes and release. Each Luna stops immediately after its coherent compiling tranche and focused tests; root commits exact paths before the next tranche. No overlapping uncommitted edits.

**Tranche 1 — EQ representation and prepared render.** Primary owner paths: `crates/parametric-eq/src/lib.rs`, `crates/parametric-eq/src/response.rs` only for the minimum compile-required response array/mapping adaptation, and `crates/parametric-eq/tests/{contract,conformance,bank,mono_collapse,analytic,time_domain,determinism}.rs` plus `tests/support/mod.rs` when helpers need to derive descriptor counts. Implement descriptor/default-target storage, six-stage ordering, correct mono/dual stationary dispatch, prepared/reset/payload updates, bypass and focused tests. No host/SDK/ABI-control edits. Use existing fixtures or tiny deterministic signals. Checkpoint gate: `cargo test -p parametric-eq` (bounded failures in older fixtures may be recorded only if root explicitly approves a useful buildable checkpoint; prefer repair in the owned test paths), and `cargo check -p parametric-eq --target wasm32-unknown-unknown` with repository target flags. Stop with paths, exact commands/results and remaining failures.

**Tranche 2 — response, catalog and downstream compatibility.** Primary owner paths: `crates/parametric-eq/src/response.rs`, `crates/parametric-eq/tests/response.rs`, `tools/parameter-metadata/tests/round_trip.rs`, `sdk/assets/miso-engine-v1-parameter-metadata.json`, `sdk/src/generated/catalog.ts`, SDK prepared-session tests, existing focused response/bank integration fixtures that fail solely because section/parameter counts changed. Generated files are refreshed by normal generators. Add exact response-ID/DSP mapping tests, 6-section total/section curves, old/default session bit regression, catalog truthful prepared-only rows and old payload refusal. Audit direct EQ_SECTION_COUNT consumers with rg and adapt only concrete necessary source/test consumers; report any expansion to root before new cross-crate design. The parallel #147 ABI child owns metadata schema structure; coordinate regeneration after root checkpoint, do not overwrite its work.

**Root evidence tranche.** Root may delegate a separate bounded read-only audit of state/floor accounting and reference updates. Owned documentation: numbered local child spec, #804 tracker, `docs/rulings/effect-floor-accounting.md` relevant EQ inventory, and a concise existing EQ implementation note or child decision record. No whole-corpus byte repinning merely to make gates green. Any new unrelated benchmark or audit harness is a successor.

## Mandatory acceptance evidence

- Exactly one prepared EQ instance includes HPF + 4 general bands + LPF; every old ID keeps its meaning; old sessions default to cuts disabled.
- Deterministic filtered PCM and existing independent response oracle cover HPF-only, LPF-only, both, disabled identity, Q/cutoff boundaries and all 4 launch rates. All output/state stays finite under existing policy. Reuse existing independent analytic/impulse authority, not a test that only repeats the implementation.
- Original-band rendered bits with cuts disabled remain identical, including scalar/SIMD/mono collapse and tail-track coverage. A 6-section regression makes the final LPF detectably affect output under scalar, Simd4, Simd8; this specifically catches remainder-dropping dispatch.
- Public response IDs and original-band meanings remain stable; HPF/LPF appear as 5/6 and totals reflect physical cascade. Effect-wide bypass returns unity for the full effect.
- Payload sizes reflect 456 bytes per channel, old shape is refused atomically, and valid snapshot/restore/reset round trips include all 6 sections. Relevant prepared memory accounting follows actual allocations.
- Metadata/SDK generator checks show all six controls and accurately keep them prepared-only. Native and Wasm builds pass. Existing realtime allocation gate passes for rendering prepared cuts; no coefficient design is added to callback paths by this child.
- Proportional commands include `cargo test -p parametric-eq`, `cargo test -p parameter-metadata`, the repo metadata generator/check, SDK `check:generated` and relevant prepared-session/type tests, plus target build. Root chooses integration checks from actual affected consumers. Run broad checks once at delivery; repeat only for new changes/failures.

Existing SVF listening provenance remains applicable to the reused second-order design. Record any new listening actually performed candidly; do not claim unheard fixtures or block an additive prepared slice on scheduling a new broad listening corpus. No descriptive benchmark is mandatory solely to close this correct product slice. If run, freeze/preflight once and use exactly one warmup/two measured rounds without tuning.

## Non-goals and successor contract

No live cut control, new generic queue ABI, variable slope, new filter family, higher general-band count, timeline/UI, hidden EQ instances, new response protocol, or dependency on unfinished #774. A separate live-EQ issue owns off-render prepared coefficient updates, enable/disable ramps, retarget/ack/backpressure tests and browser/headless SDK parity. A separate builtin issue owns live strip cuts and paired cutoff invariant. Parent #804 stays open until both live outcomes are delivered. The app uses dedicated EQ cuts only after an immutable qualified live-capable release.

## Decision/evidence record

Astra XHIGH brief PASS: frozen above from current engine 551f6d7e and live issue 804, the D2 builtin ruling, and current EQ/response source. Implementation attempt 1: pending. Astra MEDIUM adversarial verdict: pending. Root must record checkpoint hashes, focused gates, upstream issue synchronization and later final verdict here; no issue completion claim before upstream evidence.

## Attempt 1 — prepared representation checkpoint

Luna XHIGH completed the six-section representation, original-band mapping,
prepared cut descriptors, depth-two dispatch, payload codec and minimum
response/fixture adaptations. Root approved the necessary existing corpus and
response-test count adaptations. Exact changed implementation paths are
`src/{lib,corpus,response}.rs` and `tests/{contract,response}.rs`,
`tests/support/mod.rs`, all under `crates/parametric-eq/`.

Passed: package cargo check, fmt/diff hygiene, clippy over all targets/features
with `-D warnings`, Wasm target check, and the complete parametric-eq test run.
All active suites passed; existing ignored descriptive cases were not changed.
The agent unnecessarily started a duplicate final test run; it also completed
without failures, and no additional repeat is required. Evidence:
`/tmp/805-tranche1-evidence.md`.

The six-section serialized payload is 456 bytes per lane, 920 bytes total with
the common header; this is not the resident Rust object size. New HPF/LPF
stationary regressions prove both sections execute at widths1/4/8. The current
disabled-cut test compares two six-section paths; it does not independently
prove old-four-section bit identity. Tranche2 must add that direct oracle,
complete response/catalog/prepared-SDK coverage and confirm resident accounting
before the coherent attempt receives its fresh Astra MEDIUM verdict. No live
cut capability or completed #805 claim is made by this checkpoint.

## Attempt 1 — response evidence and retained failing compatibility regression

The test-only response tranche verifies public/physical section mapping, retained
and requested response totals, cut/Q/domain boundaries at all four launch rates,
and final-LPF mono collapse. Response: 18 passed; mono collapse: 3 passed.
Clippy and Wasm check passed. The independent old-four-section oracle exposed a
real compatibility failure, retained as a permanent nonignored regression:
`disabled_cuts_preserve_all_high_pass_signed_zero_against_four_section_oracle`.
With disabled dedicated cuts, four original HighPass bands at 1 kHz/Q1, 44.1 kHz,
mono cold positive-zero state and negative-zero input, the new path emits
`0x00000000` while the original four-section kernel emits `0x80000000`.
The focused compatibility suite is deliberately red (6 pass, 1 fail). This is a
buildable evidence checkpoint, not a compatibility PASS or completed attempt.
No production arithmetic changes accompany it. Astra XHIGH is specifying the
bounded correction; the hard bit-identity gate remains unchanged. The source
checkpoint that previously passed lacked this adversarial case.

## Approved narrow correction: masked SVF output

Root accepts the Astra XHIGH design below as a narrow #805 amendment. Authorized added dependency paths are exactly `crates/lane/src/kernels.rs` and one focused lane test file. The two additive helpers leave every existing unmasked consumer and the Lane trait unchanged; no general framework or other effect migration is authorized. This is necessary to satisfy the existing bit-identity gate, including mixed banks, rather than a new product outcome. Accounting glue in `tools/bench/src/floor.rs` and `scripts/console-benchmark-record-lib.jq` is authorized only if the shipped source inventory changes.

# #805 signed-zero correction — bounded design

Design recommendation for root review, based on `/tmp/miso-engine-804` at `f9d12f17`, implementation `c468afce`, and the in-progress test-only tranche. No repository changes, tests, commits, GitHub operations, or agent delegation were performed for this design.

## Decision

Keep one EQ with six physical sections and the unchanged TPT recurrence. Give only the two dedicated cuts a **per-lane, bitwise dry-output selection when settled at exact identity**. Continue executing their existing integrator recurrence when their section executes. Apply this rule inside both the stationary cascade and the per-section/ramped path, before a section's output can feed the next section.

The smallest robust implementation that also respects #807's ramp ownership is two additive lane-kernel entry points, followed by a small EQ integration. The existing unmasked lane APIs retain their behavior. This is an output-selection correction, not a change to cutoff/Q design, coefficient arithmetic, old-band enable semantics, or the recurrence.

## The failing evidence remains a failure

The permanent regression is the correct gate: at 44.1 kHz, disabled dedicated HPF -> four original HighPass sections (1 kHz, gain 0, Q 1, slope 1) -> disabled dedicated LPF; cold `+0.0` integrators and first input `-0.0`. The direct four-section `svf_block` oracle returns `0x80000000`, while the current six-section route returns `0x00000000`.

`svf_block` explicitly evaluates `m2*v2 + (m1*v1 + m0*x)` through the frozen unfused `Lane::fma` operations. Its identity coefficient words are a transfer-function identity, but their output mix adds positive zeros to `x`. The existing elision commentary correctly documents that this can clear a negative-zero sign. Refusing elision therefore restores six-section execution, **not** old-four compatibility. The six-versus-six refusal test remains useful as a scheduling test, but cannot discharge this regression.

Keep the counterexample and its failing command/result. Root should replace the rationale in `/tmp/805-tranche2a-evidence.md` with an explicit failed compatibility gate, then record the corrective checkpoint and fresh verdict. Do not erase the failed attempt or normalize the comparator.

## Exact sample and state contract

For each physical section and lane, compute a constant mask for the current stationary block or ramp segment:

```text
dry = section is HPF or LPF
   && remaining[section][lane] == 0
   && every current coefficient word has the exact IDENTITY_WORD_BITS
```

For original physical sections 1..4, `dry` is always false, including disabled original bands. A bank-wide identity flag cannot replace this per-lane mask. A target enable flag alone cannot replace it either.

The executed sample body is:

```text
x = input vector at this section
(v1, v2) = existing svf_step(x, -c1, a2, a3, mutable local state)
wet = m2.fma(v2, m1.fma(v1, m0.mul(x)))
output = Lane::select(dry, x, wet)
```

`Lane::select` is specified as bitwise. Its dry arm preserves every input bit, including signed zero and subnormals, without subtracting, multiplying, normalizing, or mixing the dry value. Enabled lanes execute exactly the previous SVF operation order. There are no horizontal operations; different enable states, cutoff/Q values, and ramp countdowns can coexist in the same bank.

The selected output must become the next section's input immediately. Selecting only after the whole six-section cascade cannot repair state already changed in an original band. Saving an entire dry block to select afterward would also violate the zero-scratch constraint and still be the wrong section boundary.

Retain the **existing recurrence state behavior** of an executed cut. Do not freeze, clear, or substitute its integrators just because output is dry. At canonical identity and finite input, an already `+0.0` state stays `+0.0`; a restored normal finite state remains subject to the same identity recurrence; signed-zero and tiny restored state receive the existing `flush` behavior. Test the full words rather than assuming that all restored state is cold. During ramps, state continues through the current coefficients without a reset. This preserves the six-section implementation's retained-state policy and the state from which a later enable begins.

Existing safe identity elision may still omit a section after proving its state would not change. Do not add a new all-disabled shortcut. In particular, do not omit a disabled cut carrying noncanonical integrator state merely because its output is dry. Existing fault/reset handling remains in place; no new NaN policy is introduced by this output selection.

When both dedicated cuts are settled disabled, each is a bitwise identity on the signal. Induction over the original four sections then gives identical original inputs, outputs, and integrator words to the direct four-section oracle. This proof holds when the current elision gate refuses, and for arbitrary mixtures in neighboring lanes.

## Kernel boundary and ownership

Relevant existing seams are `crates/lane/src/kernels.rs::svf_step`, `svf_block`, `svf_cascade_interleaved`, and `svf_block_ramped`; `Lane::select` already provides the required operation. No new `Lane` trait method or backend intrinsic is needed.

An EQ-local stationary wrapper is technically possible: it could own the frame loop, call public `svf_step`, retain the existing output-mix order, and select dry. Thus this is not blocked on access to the recurrence. However, the existing block/cascade APIs do not expose each section's original input for a mixed-lane selection, and `svf_block_ramped` explicitly owns per-sample coefficient advancement under D10. A complete local ramp implementation would either duplicate that ownership or reduce block kernels to one-frame calls. Neither is the recommended correction.

Add only these two capabilities to `lane::kernels` (names may follow local convention, unversioned):

```rust
svf_cascade_interleaved_with_dry_masks<L, const S: usize, const D: usize>(
    io, frames, coefficients, states,
    dry_masks: &[[L::Mask; D]; S],
)

svf_block_ramped_with_dry_mask<L>(
    io, frames, coefficients, step, ramp_frames, state,
    dry_mask: L::Mask,
)
```

They differ from the corresponding existing kernels only by the output selection above. The ramped helper with `ramp_frames = 0` is also the stationary per-section masked body; no third public helper is necessary. Use a small private const-specialized implementation if sharing the existing frame/update loops is needed: old entry points select the unmasked specialization, new entry points select the masked specialization. Do not route every existing consumer through an unconditional extra select. There is still one `svf_step` recurrence and one shared implementation of the ramp update law.

The initial masked cascade can perform one select per executed section, including an all-false select for original bands. That avoids a per-frame mask reduction or a new dispatch framework. It is correct and has an explicit accounting cost; removing redundant selects is not part of this fix.

**Scope action before code:** root must record this dependency-boundary correction. The conservative split is a smallest helper prerequisite issue with exactly `crates/lane/src/kernels.rs` and one focused lane test file, then #805 integration. Its closable contract is bitwise masked output with unchanged recurrence/state/ramp arithmetic; EQ is the only new consumer. An explicit narrowly approved #805 amendment can instead name exactly those paths and backward-compatible capabilities, justify why no other consumer or trait contract changes, and preserve the same gates. Do not silently expand #805 or turn this into a general filter framework. If implementation requires other effect changes, a new trait, or a changed unmasked contract, stop and split/rebrief.

## EQ integration, stationary and ramped

1. In `interleave` and `interleave_mono`, build masks using the physical indices in the selected `at` list, alongside the existing coefficient/state arrays, then call the masked cascade. Keep six-section depth 2, physical order, and the existing divisibility assertions. Original-band all-false masks are independent of their coefficient values.
2. In `Channel::process_section`, keep original sections on the existing `svf_block`/`svf_block_ramped` route. For dedicated sections, call the masked block helper; use zero ramp frames on a stationary segment. Recompute its mask after the existing snap handling and before each segment. A ramp ending in only one lane must change only that lane's next segment mask.
3. `process_channels`, `process_channels_mono`, and direct `Channel::process_block` must all reach the correction. Correcting the stationary cascade alone leaves the corpus and any bank forced onto the fallback by an original-band ramp broken.
4. Keep the existing conservative elision predicates and padding rule. In their admitted domain the added selection agrees with the existing identity proof. A refusal now executes the correct masked six-section semantics. Update comments/oracles to distinguish old-four compatibility from agreement between the two corrected six-section schedules.

Derive masks from current coefficient **bits** and `remaining`, rather than adding a stored enable mask or reading `targets.enabled` in the render body. The current restore validator requires settled coefficients to equal the designed target exactly, and canonical disabled cut targets design to identity. This derived mask is therefore correct for preparation and restore, and does not create a new member of the designed-channel symmetry read surface. Existing `coef` and `remaining` comparisons already cover it. Extract bits with bounded fixed arrays or the existing helpers, then create a canonical `Lane::Mask` by comparing a vector of 0/1 decisions; do not synthesize arbitrary mask representations or use floating equality as a substitute for bit equality.

All persistent ownership stays with the existing `Channel`/`PreparedParametricEq`; no new state cache or serialized field is required. Temporary masks and coefficient/state copies are fixed-size stack values bounded by S <= 2, D = 2, six physical sections and W <= 8. No PCM scratch, new heap storage, allocation/free, I/O, lock, syscall, parameter-based cohorts, dynamic dispatch, or unbounded loop is introduced. Section splitting remains bounded by the existing finite per-lane ramp ends. Resident object size and the #805 456-byte lane payload remain unchanged for this correction; recount rather than assume if implementation adds fields.

## #807 continuation and one existing discrepancy

The same helper and mask rule survive live enable/disable. A disable **target** cannot turn on dry selection while a ramp is in flight. On the exact settled-identity boundary, dry selection begins. Enable and retarget clear the settled condition; coefficient/state evolution continues normally. An idle disabled lane remains dry even while another lane ramps. Refreshing masks per segment handles staggered completion and block partitions. #805's six rows remain prepared-only throughout this correction.

Preserve #805's existing original-band ramp behavior now. There is a pre-existing discrepancy to resolve explicitly in #807: its spec promises current-then-advance, A+64 target use; `Channel::process_section` currently calls `advance_words` before the ramped kernel, passes `length - 1`, and snaps when `remaining == 1`. Consequently its first sample uses an advanced coefficient and its 64th sample uses the exact target. The lane kernel itself is current-then-advance. This is observable timing, not prose that may be silently reconciled.

#807 must change the application/segment driver to its frozen 64-update convention, keeping coefficient updates in the lane helper and exact final assignment at the segment boundary. Its boundary oracle must check sample A, all 64 updates, first exact-target sample A+64, retarget, and uneven partitions. Do not disguise that future driver change as this signed-zero fix, or change old-band timing while trying to make #805 green. The dry-mask rule applies to the actual settled boundary under either driver and need not be discarded.

## Accounting and claims

The existing EQ floor text still describes four physical sections and `24 * kept + 3` lane-ops. Update it to distinguish four general bands from six physical sections, identity across all relevant bank lanes/channels from the number of user-enabled controls, and actual depth-2 padding. A mixed cut remains executed even when some lanes are dry. Elision refusal executes six sections.

For the simple masked stationary cascade specified above, every retained section executes 24 existing operations plus one `Lane::select`, so its inventory is `25 * kept + 3`, with `kept = 2 * ceil(live / 2)` when the existing gate admits, otherwise 6. Here `live` counts physical sections that are not identity across the whole required bank/channel set. Thus the standing one-live-band/two-kept-section inventory becomes 53 rather than 51, and a full six-section pass is 153. These are source operation counts, not timing measurements. Derivation must follow the actual shipped implementation if its specialization removes selects.

Update the relevant root-owned ruling and its named live constants in `tools/bench/src/floor.rs` and `scripts/console-benchmark-record-lib.jq` as accounting glue; do not alter historic raw records or claim a new percentage against an old timing. Mask construction is a bounded block/segment control cost, to name separately from per-sample arithmetic. Ramped paths retain coefficient-add costs as before and gain the dedicated-cut selects. No benchmark invocation, optimization round, projected win, or new performance framework is required.

## Two bounded Luna assignments

**Assignment 1: additive kernel capability.** Own only `crates/lane/src/kernels.rs` and one focused existing/new lane test file under the recorded prerequisite/amendment. Add the two output-mask entry points with no other consumer changes. Gates: all-false masks reproduce existing unmasked output, state and coefficient bits; mixed masks match independent scalar lane executions; dry outputs preserve `-0.0`; executed dry-state updates match the existing recurrence including signed-zero/tiny/normal seeded state; cascade order and per-section dry selection are tested with a downstream section that exposes the original counterexample; ramped mask cases preserve current-then-advance, final coefficient bits, zero-increment lanes, and split-block equivalence. Use widths 1/4/8 and S=1/2, D=2. Run focused lane tests, existing SVF G2 identity gates, and lane native/Wasm checks. Stop for root's exact-path checkpoint. A downstream test cannot be declared green until Assignment 2 integrates the helper.

**Assignment 2: EQ consumer correction and compatibility gates.** Own `crates/parametric-eq/src/lib.rs` plus the minimum existing `tests/mono_collapse.rs`/`tests/bank.rs` changes. Keep the permanent all-HighPass failure and make it pass in stationary and forced fallback, mono/dual, scalar/Simd4/Simd8. Compare output and every original-band state word directly with the four-section oracle. Include old general-band ramps while both cuts stay disabled. A small mixed-bank matrix covers neither/HPF-only/LPF-only/both per lane, asymmetric channels, independent cutoff/Q values, and tail placement; compare each bank lane to its scalar instance, with exact bits and a nontrivial final LPF. Cover restored noncold dedicated state and existing elision refusals, then verify six-section schedule equality. Use tiny signals and existing fixtures, not a new corpus. Run `cargo test --locked -p parametric-eq`, package clippy, and the repository-flags Wasm check; root runs the existing relevant realtime/artifact gate at the coherent boundary. Stop for root checkpoint and fresh adversarial verdict.

Preserve existing corpus pins and old-band arithmetic. If they change, investigate the change against the old-four oracle before touching a digest. Existing six-versus-six tests should continue to compare corrected paths; they must not keep a stale unmasked cut oracle as the product authority. Root owns the issue attempt count, the old failed evidence, floor/state documentation, proportional delivery checks, and GitHub synchronization.

## Attempt 1 — masked helper prerequisite checkpoint

Luna XHIGH added only the two approved additive helpers in lane kernels and one
focused test file. Private static output policy preserves unmasked entrypoints;
masked output selects input bits at each section boundary after unchanged
recurrence/state updates. Focused tests cover widths1/4/8, streams1/2, depth2,
false-mask equivalence, mixed scalar parity, signed zero/seeded state, cascade
boundaries and current-then-advance ramp/partition coefficient bits.
Passed: all active lane tests (two existing descriptive ignores), release masked
plus G2 tests, clippy, native release build, scalar and SIMD Wasm builds, lane
policy, unfused seal and formatting. Evidence: /tmp/805-mask-kernel-evidence.md.
The existing EQ consumer is deliberately unchanged and its retained regression
remains red until the next bounded integration. No final verdict yet.

## Attempt 1 — EQ correction and accounting checkpoint

The dedicated per-lane dry masks now follow exact current identity bits and
remaining==0 at the section boundary in stationary mono/dual and fallback
rendering. Original general bands remain wet and retain their old ramp timing.
The permanent old-four all-HighPass negative-zero regression now passes.
Passed: full parametric-eq suite, package clippy, scalar and SIMD Wasm checks,
fmt/diff checks. /tmp/805-eq-mask-evidence.md records commands. Luna created
commit1aee3d60 despite the root-only commit instruction; root audited and
preserved this coherent source checkpoint without rewriting history.

Root applied the separate Luna accounting draft and verified nine focused bench
floor tests, including Rust/jq exact parity; no benchmark timing was run. The
current stationary inventory is25*kept+3 (53 standing two-kept,153 full six),
with mask construction accounted separately as bounded segment/block work.
Payload remains456bytes/lane,920serialized total, not resident allocation size.
Root rejected repricing historical timing percentages against the new mask
inventory: those tables retain their historical51-op EQ floor/percentages and
are explicitly labelled historical. No new measured performance claim follows.

A small final compatibility test supplement remains for mixed enabled cuts and
original-band ramps, followed by catalog/prepared-SDK evidence and the fresh
Astra MEDIUM complete-attempt verdict. #805 is not complete yet.

## Attempt 1 — final compatibility and artifact checkpoint

The compact supplement passes mixed cut enable states/asymmetric parameters at
Simd4/Simd8 with scalar parity and a scalar tail, disabled cuts around an
original-band ramp against the direct four-section oracle at all three widths,
and restored tiny cut-state flushing. Corrected the restore probe offset to
physical band1 and stale six-section prose identified by Astra.
Passed: 11 interleave tests, the restored-zero contract probe, full EQ package
suite, clippy, scalar/SIMD Wasm checks and formatting. Evidence:
/tmp/805-final-compat-evidence.md. Native size_of Channel=656/2608/5216 bytes
and PreparedParametricEq=1832/6608/12992 at widths1/4/8, excluding allocator
overhead; serialized total remains920. Existing host exact-budget-minus-one
gate passed (/tmp/805-host-memory-cap.log); its cap is declared serialized
state, not all resident heap allocations.

Root built matching Wasm twice through the existing repin/ordinary builder:
b0bff10d67bd39e46ea6a4d102b550c0abbf26607e898face54523e269d39765,
artifacts /tmp/804-805-artifacts. Ordinary build and check-web-audioworklet
passed, including render allocation/callgraph and SIMD shape gates (EQ dual
252vector/0scalar, mono126/0). Render-contract gate passed. Logs:
/tmp/805-wasm-build.log, /tmp/805-artifact-check.log,
/tmp/805-render-contract.log. No new npm release yet.
Astra's committed DSP review has no production blocker; final verdict waits
for generated catalog and prepared SDK/package evidence.

## Attempt 1 — catalog checkpoint and exposed response integration failures

Normal Rust metadata/SDK generators now expose30EQrows: original spaced IDs
unchanged and six prepared-only cut rows. Typed booleans serialize0/1; omitted
cuts preserve sparse params; live types still exclude new cuts. Passed:
parameter-metadata (5round-trip,7ABI), generated/schema/types/fmt checks and
the new builder case. Evidence /tmp/805-catalog-evidence.md.
The required full headless run is explicitly RED:253tests,250pass,3fail.
response-evals still expects4sections, while live-response and subscription
queries hit host invalidArgument. Root located obsolete count>4 bounds in
host-web FFI and both SDK live-response parsers. No failures are waived.
Astra XHIGH is defining the minimum bounded6section transport correction and
malformed-payload defenses before implementation. This is the existing response
compatibility requirement; no new live-cut capability is claimed.
