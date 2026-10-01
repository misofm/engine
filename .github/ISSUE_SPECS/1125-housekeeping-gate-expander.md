# Housekeeping: gate-expander

## Authorized scope and smallest closable slice

Review the complete `crates/gate-expander` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/gate-expander` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker A, GPT-6.1 Sol xhigh, 2026-10-01

Read all 15 package files and the related #894 (silent admission), #973 (mono collapse), #1073
(D7 report units), #559 gate findings and original product/recovery briefs. Preserve those scopes,
the independent current-law f64 model and G5's sole digest ownership. Root approved the bounded
cleanup: skip already-validated bank member 0, remove the unused private commit byte argument,
borrow repeated test PCM columns and continue the unrestored donor, hoist identical W4 generator
words, remove the permanently false padded-state branch and three weaker tests with stronger
public/width survivors. Correct current arithmetic/scenario and test-owner documentation in the
same tranche. Baseline locked all-feature package tests: 49 PASS, 2 ignored (W4 smoke and #1073).
No dependency/lock, DSP expression, API, feature, generator/schedule or G5 change is authorized.
Root committed the product tranche as `16c32c8e`, integrated/pushed as `a657a3f8`; the one unused
import and missing article were checkpointed as `507320b5`, integrated/pushed as `cc4b7dd5`.

**Five axes.** Repetition: omit member 0's second metadata/default validation after it already seeded
the bank; all remaining member/padding validation still precedes program/sidechain/width fallback.
Remove the unused private restore byte argument, permanently false padded-state branch and its
unused preparation/snapshot, and three weaker tests. Copies: `packed` borrows source slices and
constant planes instead of cloning columns before interleaving; rollback owns its existing payload;
active continuation uses the never-restored donor instead of preparing/replaying its prefix.
Hoist first-block bank spans/offsets outside the block loop and generate each W4 continuation noise
column once (4 calls instead of 448 at 112 frames), with identical seeds, words and schedule. Read
test state sizes from the bank's sealed key; scan corpus words without another converted Vec.
Independent mutable render arms, corrupted payload copy and atomic two-channel restore staging
remain necessary. SIMD: the causal kernel already uses one `Lane` graph for level/link, inclusive
phase/hold, curve, rate recurrence, ramp snap and output. Retain its unfused multiplication/addition,
W4/W8/scalar/tails and D8 NaN order verbatim; no new vectorization or generated-code/speed claim.
Structures: bounded arrays (two channels, four ramps, seven parameters, eight scratch lanes) and
member bitmask earn fixed work; no replacement earns an evidence-based benefit in this slice.
The wider constant-mask/shared-helper/parameter-table proposals remain #559; #894 silent admission
and #973 mono collapse need their separate proofs. Net package **−115 lines** (Rust −123, mutation
survivor notice +8), ten package paths, no manifest/lock/G5/arithmetic/API/feature change or timing.

**Complete retained family purposes.** Counts below describe this native AVX2/FMA run.

| Family | Plausible defect defended |
| --- | --- |
| parameter-spec unit (1) | Descriptor/runtime domain, mapping, defaults or 64-sample smoothing drift. |
| private gain-fault unit (1) | Nonfinite recursive gain hidden by finite fast-gain output: native bank/scalar parity, uninjected peers, channel-local reset/report and canonical Open/K/+0. |
| internal W4 continuation unit (1) | W4 nonzero gain and serialized continuation diverge from an unrestored donor, including PCM and payload. Deterministic noise hoist preserves each frame/lane word. |
| padding differential (2) | Both widths, every partial count, links and launch-rate inputs: random parameters/Points/signed zeros/subnormals/bursts plus all seven conformance fixtures match scalar PCM, reports and active state. Padded lanes ignore stray valid/invalid spans, emit +0/report nothing and match an independently advancing idle scalar's finite state each block. Fixture schedule is a repeating 128/64/37/1 sequence, unchanged. |
| padding domains and clone source (2) | Full/partial prepared payloads remain in domain and restore into scalar; domain-extreme/drawn clone choices cannot change active PCM. Removing the false constant-state assertion retains actual idle-state comparison. |
| padding fault isolation (2) | At both widths, planted NaN gain in an active or padded lane resets only its channel and reports only active lanes; 1e30/inf/NaN input trips a nonvacuous identity-range lane while every peer retains control PCM/reports. |
| contract metadata/resources (2) | All four launch rates, stable IDs/layout/compact format/zero latency and tail; exact state/scratch admission and one-below typed refusal. Format counts describe the existing payload, not a new memory pin. |
| contract payload (1) | Version/count header and malformed final-right-word rejection leave both channels untouched; current-format snapshot succeeds after discontinuity. Owning the prior payload removes three copies without changing the check. |
| contract initial admission (3) | Extra, missing, misordered, wrong-channel, negative-zero and subnormal initial records refuse with current typed errors. |
| contract bank admission (2) | Non-first malformed member refuses before fallback; public native full/every padded count binds and validates malformed active and padded clones. These cover member-0 skip and preserve refusal order. |
| contract bypass and unity (3) | Bypass, enabled ratio-one and enabled zero-range return nonzero sample zero immediately at every launch rate; latency stays zero. |
| contract range floor (1) | Independent numerical settled 48 dB floor is nonzero and clamped, where the f64 interior-curve fixture deliberately stays off the clamp. |
| contract current-sample causality (2) | Changed future main suffix inside the same API block cannot move an earlier output; explicit closed gain/phase followed by trigger proves first attack occurs on the trigger sample. |
| contract hold/boundaries (3) | Hold zero closes immediately; nonzero K remains open for exactly K below-band samples, then closes and retriggers; exact opening/rearm equality with both adjacent comparands catches strict/inclusive drift and a misplaced constructed equality. |
| contract connected sidechain (4) | Sidechain override/current quiet-word closing; valid bank decline and scalar fallback; future sidechain suffix within the same block cannot move current output; NaN detector keeps current finite/no-report policy. |
| identity (4) | Lane-distinct native scalar/bank PCM and full state across all links/Points; real 1/63/64/65/127/128/129 partitions across 64-update ramp snap; bypass preserves signed-zero bits while gain advances; equal channels remain equal and zero input after history has no tail/report. |
| state (7) | Both resets with live gain/four live ramps and retained versus original timing/defaults; active donor continuation at remaining=47; obsolete/short lengths at all launch rates; malformed-right scalar/bank rollback with peer PCM/state; channel/lane-local scalar/bank D7 versus control/fresh state; scalar↔bank payload interchange; one-track bank restore leaves peer payload intact. |
| oracle (3) | Independent current-law f64 curve/phase/envelope on scalar and native bank for all links, fixed 0.02 dB limit, exact open identity and active/interior/quiet nonvacuity; separate threshold-clearance gate prevents numeric tolerance from disguising decision timing. Current fast-tier/unfused documentation replaces stale exact-tier wording; model, limit and inputs are unchanged. |
| determinism (1) | Every unchanged G5 corpus case is finite and more than one-quarter nonzero; scanning words retains both predicates. G5 alone owns all unchanged target/width pins. |
| conformance (1) | Independent public factory descriptor/preparation/automation/state/fault and warmed audited zero-allocation contract harness. |
| randomized (1 + 1 ignored) | Existing 24 seeds ×24 blocks compare current scalar/bank/chunked/continued/restored paths with hostile payloads and all bind outcomes. Untouched #1073 fixed-input counter-unit reproducer remains ignored. |

**Deletion survivors.** The bypass-only `causal_kernel_reads_the_current_sample` could not test its
claimed detector causality; public causal/first-attack, all-rate bypass and signed-zero/advancing-gain
gates retain its identity defect coverage. `connected_sidechain_uses_current_detector_word` checked
only finite main output/no left fault; actual override/closing, fallback and NaN gates retain that
coverage and discriminate ignored/delayed detectors. The W4 binding smoke had no output/state
assertion and was ignored here; current padding differentials bind publicly on W4 targets and
internally on AVX, with full PCM/report/state checks. Thus two active and one ignored tests retire;
no independent numeric, boundary, partition, state, RT, generator or real fault gate was removed.
Retained bypass payload word 0 is gain (not a codec header), so its advancement witness stays.
Renamed native-bank/continuation tests accurately describe the unchanged schedules.

**Actual gates.** `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a` throughout.
`cargo test --locked -p gate-expander --all-features`: baseline **49 PASS + 2 ignored**; candidate
debug and `--release`: **47 PASS + 1 ignored each** (9 unit +38 integration; #1073 alone ignored).
Candidate debug exposed one unused `LinkMode` import after the weak test deletion; root checkpointed
its removal. Final strict `cargo clippy --locked -p gate-expander --all-targets --all-features --
-D warnings` and release/targets report no warnings. Package fmt/check and diff/check PASS.
Workspace, effect-runtime and realtime policies PASS. Locked library `cargo check` PASS for
`wasm32-unknown-unknown` with `RUSTFLAGS='-C target-feature=+simd128'`, `aarch64-apple-ios` and
`aarch64-linux-android`. Logs: `/tmp/housekeeping-a-gate-expander-{baseline,attempt1-debug,
attempt1-release,attempt1-clippy,attempt1-policy,attempt1-wasm,attempt1-ios,attempt1-android}.log`.

Compile checks do not claim mobile hardware execution or Wasm runtime replay. No ignored repro,
new harness/fixture, extra sweep/mutation matrix, benchmark or listening run. #1073's current
frame-count reports remain frozen pending its separate contract repair. No new owner product or
architecture question. Worker pauses for root's one adversarial verdict.

## Root adversarial verdict: PASS — attempt 1

Root reviewed every product/test/doc hunk, bank admission order and deletion survivor, plus the complete family record. Member 0 is already validated and seeded before the skipped loop; all later members are still validated before fallback. No kernel expression, mutable render ownership, ABI, state format or numeric limit changes. Borrowed columns, hoisted noise and spans preserve all frame/lane words and event schedules. The deleted bypass-only, finite-only and assertion-free binding tests have stronger current survivors named above; the real ignored fault reproducer remains.

Rewritten-test purposes: the scalar donor continuation catches failure to restore the four live ramps at remaining=47 together with gain/phase/hold, which bank interchange and malformed-restore tests do not establish; the W4 continuation catches width-specific live recursive-state/serialization drift outside the public native-width path; padded-state differentials catch a clone lane advancing differently from an idle scalar, which active-lane parity cannot detect; the native-bank independent f64 oracle catches shared scalar/bank curve drift that bit-parity alone cannot detect. Other changed tests retain their exact assertions, generators and scopes through ownership/packing changes or honest naming; the table records their distinct boundary/fault/oracle roles. No new test is added and no replacement historical implementation or digest pin is introduced.

Root integrated locked all-feature multiband-compressor, gate-expander and builtins-compiler tests: 170 active PASS, five existing ignores, no failed test (log /tmp/engine-housekeeping-multiband-gate-compiler-tests.log). Final package strict lint and native debug/release/three target compile gates pass. Rust reduction is 123 lines; two active and one previously ignored tests retire. No measured CPU, fresh SIMD, listening or all-target runtime claim. PASS covers this bounded housekeeping; required qualification still controls main integration.
