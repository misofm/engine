# Qualify causal multiband-compressor CPU cost on an active-input workload

## Status, dependency and smallest outcome

Astra medium read-only scope PASS revalidated against actual #746 delivered-candidate source ce75a69bebe5fb6a1aa55d0c6f1a1c9ea7f05a77 in `/home/bl/misofm/engine-gate-active-746`. This is conditional activation approval, not a claim that #746 has merged or its final verification passed. Root reported the sole #746 measurement passed; Astra final measurement verification and remote delivery remain prerequisites. Inspected AGENTS.md, numbered #746/#748 specs, actual Rust subject/dispatch, Python runner, multiband implementation, public effect-contract APIs and shared digest support. #739 delivered the causal product independently. Luna xhigh implements; Astra medium verifies; root owns checkpoints, invocation, evidence, GitHub, required CI and cleanup. Maximum5 coherent attempts. Latest user model routing supersedes historical names.

Smallest closable outcome: actual prepared scalar and native W8 causal multiband cost at48kHz/128frames, with fresh bounded input and separately proven low/high envelope engagement and release for every track/channel. Reuse #746's one-warmup/two-measured-round runner and provenance; do not create a second persistence, PMU, host-metadata or benchmark framework. No DSP change, sound-quality judgment, old-lookahead baseline, speedup, floor or capacity claim.

## Existing evidence and allowed paths

Current `crates/multiband-compressor/tests/descriptive_frame_cost.rs` feeds processed output back into processing and retains the best of two measured rounds, with historical before/after labels. Do not run or cite it. The private split benchmark compares split on/off and is not this product workload. Neither needs repair for this slice.

Reuse the delivered #746 tools/bench dispatch, public registry lookup, bench-support timing/JSON/metadata, Python runner and hermetic runner tests. Exact allowed paths after revalidation:

- `tools/bench/src/main.rs`: native `multiband-active` subject registration.
- `tools/bench/src/multiband_active.rs`: one bounded subject and its effect-specific activity validation/tests.
- `tools/bench/src/gate_active.rs`: visibility only for existing `Phase` and its parse/name/blocks/timed/round methods, plus `ReportCounts`, its existing fields and add/is_clear methods, to `pub(crate)` as needed by the new sibling module. No extraction and no gate fixture, validation, record, process-loop or measurement behavior change.
- `scripts/run-gate-active-benchmark.py`: explicit two-choice `--subject gate-active|multiband-active`, defaultgate-active, dispatching to exactly two known schemas/activity validators. Retain filename for compatibility; no generic arbitrary subject/count/plugin interface.
- `scripts/test-gate-active-benchmark.py`: subject-selection, wrong-effect schema/activity and preserved default-gate behavior tests, using existing stub phase/perf mechanism.
- `.github/ISSUE_SPECS/748-qualify-causal-multiband-active-input-cpu-cost.md`: root scope/evidence record.

The actual gate subject has private, gate-specific preparation/process/activity/record functions, not a reusable generic prepared-row engine. Reuse its phase/count parser and report accumulator through the above bounded visibility adjustment; reuse the existing bench-support timer, percentiles, metadata and digest APIs directly. Keep multiband-specific prepare, scalar/bank process loops, snapshots, activity and record assembly in the new subject. Two short explicit process loops are justified by different scalar/bank APIs and snapshot calls; do not copy the entire gate module or introduce a trait/configuration-driven generic workload framework. No file extraction is justified or authorized in this slice. Do not rerun #746 timing to test reuse. Its untimed tests are the regression evidence.

In Python keep one shared orchestration and normalization/report/elapsed validator. Thread a validated explicit subject choice through preflight, run, record validation and issue labeling (including temporary paths, manifest, status and printed summaries). Preserve the existing default gate issue746/record gate_active behavior. The new branch requires issue748/record multiband_active and effect_id miso.multiband-compressor; it must not impose a new effect_id field on existing gate records. Branch only identity and activity/workload-specific validation; reuse counter, source/host metadata, child launch, enrichment, persistence, drift and failure-stop functions without another framework. Existing gate records contain no input digest; add the required digest only to multiband records using existing bench-support support, not by changing gate records.

No manifest/dependency/lock changes, production effects/graph/state changes, broad console-workload edits, old benchmark repair, historical resealing, artifact/SDK pins, browser/extended-rate matrix or optimization. If #746 lands different paths or contracts, amend this scope against that delivery before implementation; do not treat this plan as proof of existing APIs.

## Frozen multiband workload

48,000Hz,128frames, Normal, enabled, DualMono, no sidechain, no automation. Prepare actual W1 and bind actual matching native AVX2 W8 through `launch_native_effect_registry` and effect ID `miso.multiband-compressor`; reject fallback/missing bank. Assert latency0, empty common section,188bytes per channel and scratch0 as workload identity. No internal kernel substitute.

Concrete preparation API: multiband declares only main input/output ports, so construct `PreparedPorts { sidechain: PreparedSidechainPort::None }`; do not reuse gate's request helper, which demands sidechain-in. `validate_prepare_request` rejects zero admission capacities even for zero actual scratch: set maximum_scratch_bytes=1, maximum_total_state_bytes=376 and maximum_automation_spans_per_block=16; assert prepared actual scratch_bytes=0. The one-byte admission ceiling is not measured scratch consumption. Scalar metadata.state_sizes and bank metadata.program_key.state_sizes must both equal common0/left188/right188, with state_layout_version1, and all other frozen metadata checked. Bank metadata is PreparedBankMetadata { width, program_key }; snapshot sizes come from program_key, not an invented bank effect_metadata accessor.

Resolve descriptor compact indices from stable IDs. Crossover ID1=1000Hz. Low IDs3/4/5/6/7: threshold=-30dB, ratio=4, attack=1ms, release=5ms, makeup=0dB. High IDs8/9/10/11/12 have the same threshold/ratio/attack/release/makeup values. ID2 remains retired. Both channels/tracks use these parameters; signal phases vary to keep independent state active.

Generate fresh input before each timed process call from exact binary-valued square-wave components. For absolute sample n, track t, channel c define m=n+24*t+12*c. Let low(m)=+1 when m%384<192 and-1 otherwise (125Hz); high(m)=+1 when m%12<6 and-1 otherwise (4kHz). Define plateau=(block+8*t+32*c)%64. Input is A*(low+high), with A=1/4 for plateau0..31 and A=1/4096 for plateau32..63. This is bounded by0.5, bit-exact f32, and needs no RNG/transcendentals or allocation. Individual samples may cancel to0; every plateau contains nonzero energy. Components have strong fundamentals well below/above1kHz and both continue at a small nonzero amplitude during release. This is an activity stimulus, not a spectrally pure scientific fixture or new crossover oracle.

Fill planar scalar or AoSoA bank buffers fresh each block; never reuse processed output as input. Record the exact generator/counts and an input digest outside the timer through `bench_support::digest::Sha256Sink::{new,update,finish_hex}`. Freeze digest serialization as block-major, frame-major, track-major, channel-left-then-right, each source sample's f32 little-endian bytes before processing. Reuse a preallocated block byte buffer and one update per block; no new hash implementation/dependency. The source generator uses n=block*128+frame independently for each phase. Record the generator identity, parameters and state sizes so the activity schema has an explicit workload identity. Output must remain finite and have nonzero energy; actual process reports for every lane/block must show zero faults and invalid spans.

For each channel/track read the actual saved payload's two distinct gain words through public scalar/bank snapshot hooks outside timing. Current layout word1 is low and word2 high. Use preallocated188-byte sections, check declared sizes, and decode little-endian f32. Concrete public hooks are `PreparedNativeEffect::snapshot_state_payload(&self, StatePayloadOutput<'_>) -> Result<(), StatePayloadError>` and `PreparedNativeEffectBank::snapshot_track_state_payload(&self, track_index: u32, StatePayloadOutput<'_>) -> Result<(), StatePayloadError>`. Construct output with `StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes)` using reusable arrays [u8;0], [u8;188], [u8;188], propagating constructor/snapshot failures. Sequentially reuse this one pair for each bank track outside timing; never allocate per snapshot. Gain offsets are bytes4..8 for low and8..12 for high; decode through f32::from_le_bytes, require finite values and reject positive gain. Snapshot at each track's completed plateau boundary (phase31 or63), then observe both channels' applicable high/quiet phase; no need to snapshot every non-boundary block. The public observation gives only the deeper envelope and cannot establish both-band activity, so it is insufficient here. At completed high plateau (phase31), require BOTH gain values<-3dB. At completed quiet plateau (phase63), require BOTH>-0.1dB and<=0. Record separate high-plateau gain minimum/maximum and quiet-plateau recovery minimum/maximum and witness counts per band/channel/track. Require the high maximum<-3 and quiet minimum>-0.1 with quiet maximum<=0, so every witnessed endpoint satisfies the bounds, not merely the best witness. Each high/quiet witness count is exactly blocks/64 for each band/channel/track. Require complete unique (track,channel,band) identity coverage. The source intentionally includes zero samples; permit individual input/output zero samples, but require finite_output_samples=blocks*128 and positive input/output energy and nonzero sample counts per channel/track, bounded by that sample count. Do not reuse gate's all-samples-nonzero validator. Bounds are generous activity checks:32blocks is85.33ms, exceeding1ms attack and5ms release by wide margins. They are not relaxed DSP numerical tolerances. The untimed preflight must prove the actual fixture meets them before timing; if it fails, preserve that finding and obtain a scoped fixture amendment before changing the frozen stimulus, never tune after measurement.

## Timing, counts and reuse

Use exactly #746 counts/modes: preflight128untimed blocks per width; warmup8192blocks per width; measured rounds1/2 each32768blocks per width; scalar beforeW8. Fresh prepared instance per phase/width, numbering starts0. One root runner invocation launches exactly warmup,1,2. Every process interval uses the existing bench_support timing boundary. Filling, snapshots, report/activity collection, digesting, metadata, I/O and preparation remain outside. Retain both rounds, not min/best-of. Do not add a no-effect subtraction or unmeasured timer correction.

Extend runner with explicit subject selection while preserving defaultgate-active behavior and its already-frozen args/output refusal. Subject CLI remains `bench multiband-active --preflight` or `--phase warmup|1|2`; all extra/duplicate/unknown arguments reject before workload. Runner invocation uses the existing arguments plus `--subject multiband-active`; widths/counts/rate are frozen, not configurable.

Reuse exact #746 output creation, preflight, failure propagation and counter behavior. Root chooses a fresh absolute output directory outside disposable worktrees, e.g. `/tmp/issue748-measurement-<full-source-commit>`, freezes committed source/binary hashes, and runs no-timing preflight before the sole invocation. Preserve raw argv/stdout/stderr/exits/perfCSV and manifest/status on failure. No restart or new output directory to hide a failed measured run.

Same explicit root-owned `--perf-executable ABS` wrapper is permitted: it may execute `sudo -n /usr/bin/perf "$@"` under existing privilege, with wrapper/perf identity and expanded logical command preserved. Preflight and timing must select the same executable. No implicit sudo fallback, password prompts or sysctl/cgroup/governor changes. Actual child metadata must survive sudo filtering explicitly. No TSC or advertisedGHz substitution.

Same units as#746: lane_samples=blocks*128*2*width; mean process ns/lane-sample=sum(process_elapsed_ns)/lane_samples. cycles_per_lane_sample=mean ns * measured hardware-counter-derived effective coreHz /1e9, using cycles/task-clock and the existing3% phase-vs-warmup clock-consistency gate. State clearly that this is wall-render-time conversion by measured effective clock, not PMU-per-call counts or isolated floor accounting. Report raw ns, clock/provenance and derived cycles for all four measured rows; no unsupported floor/gap/speedup column.

## Preflight and adversarial gates

The shared runner must accept exactly round1/2 x width1/8 for the chosen subject and reject mismatched effect ID/schema/activity/counts. A gate row must not pass as multiband merely because time/width fields fit. Require separate low/high gain evidence; a single deeper-band observation, missing band/channel/lane, constant gain, no quiet phase, all-quiet input, bypass/identity or nonzero/missing fault records fails.

Before timing prove through untimed subject tests/preflight that actual both-band engagement/release succeeds and that effect-specific activity negatives fail. Shared hermetic tests preserve #746 defaultgate behavior, wrong/unknown subject refusal without launch, output overwrite refusal, exactly-one-warmup/two-rounds, failure-stop semantics, schema normalization and raw persistence. No duplicated whole failure matrix beyond new subject-discrimination cases. Run:

- focused `cargo test --locked -p bench multiband_active` and any touched gate/shared untimed tests;
- `python3 -B scripts/test-gate-active-benchmark.py`;
- locked bench check/strict Clippy, fmt/diff check and existing bench-policy check/self-test;
- release bench build, then root runner `--preflight --subject multiband-active ...` using the actual selected perf wrapper and final output destination.

Luna stops for root checkpoint when source/focused preflight tests are green. Astra medium verifies frozen fixture/runner/preflight before root timing, then validates actual retained measurements as one coherent attempt verdict. Proportional full local gates run once after source freezes, repeated only for edits/failures. No timed gate rerun, new benchmark matrix or broad production qualification.

## Delivery and limits

Root activates only after all these concrete conditions hold:

1. Astra medium final verification for #746 is PASS; its evidence is upstream, exact PR/resulting-main required qualification passes, GitHub746 is verified CLOSED and prior issue-boundary synchronization is complete.
2. Fetch and record actual resulting main SHA. Compare that main against ce75a69bebe5fb6a1aa55d0c6f1a1c9ea7f05a77 across the four #746 implementation files, tools/bench-support, crates/multiband-compressor, crates/effect-contract, crates/effect-compiler, relevant manifests/lock and target/bench-policy configuration. An empty relevant source diff preserves this conditional scope PASS; any changed signature, runner contract or workload fact requires Astra medium read-only amendment before Luna starts. Evidence-only spec changes do not require rerunning any workload.
3. Install this full scope in the existing numbered748 spec, synchronize its exact GitHub number/title/body in the same checkpoint, and replace its stale queued model-routing paragraph with Astra medium scope, Luna xhigh implementation, Astra medium verification. Confirm the issue-boundary spec/GitHub audit before implementation.
4. Start one clean dedicated feature worktree/branch from the recorded synchronized main, retain one implementation tranche, and have root commit the approved exact paths before further implementation. Follow the currently authorized push/batch delivery mode; this scope does not change push policy.

The previously queued plan's generic bank-state assumptions have been replaced above with actual delivered signatures. No production API addition, sidechain support, generic Rust extraction or dependency is needed. Numerical CPU qualification requires successful real activity/counter/record evidence; preflight refusal or post-measurement tooling failure is not measurement PASS. Any runner defect after workload preserves raw output and ends timing, with repair/requalification in a separately bounded successor if required. Required exact PR/mainCI and upstream evidence precede verified748closure and clean worktree removal. This issue supplies no human listening or sonic-superiority claim;#26 remains that boundary.

No tests, gate reruns, DSP processing, timing or repository edits were performed during this read-only revalidation. Only this temporary stateless scope artifact was written. Fixed multiband stimulus/counts and both-band snapshot requirement remain unchanged.

## Activation on delivered main — 2026-09-11

Root verified #746 Astra medium attempt2 PASS, upstream evidence, PR751 merged
as06167eb286f2d6ee46e82daf86936a24801347ee, exact PR34625139373 and resulting
main34625814610 qualificationPASS, GitHubCLOSED and clean worktree removal.
Root compared actual main against scoped candidatece75a69b across all four746
implementation files, bench-support, multiband-compressor, effect-contract,
effect-compiler, manifests/lock, target config and bench-policy scripts: empty
diff. Conditional Astra scope PASS therefore applies without API amendment.
Issue-boundary audit found no missing numbered GitHub issue. Root installs and
synchronizes this full existing748scope before Luna xhigh starts on dedicated
codex/multiband-active-cpu-748. No multiband timing has run; gate's sole run is
consumed and must not be repeated. Root retains checkpoint-push delivery mode.
