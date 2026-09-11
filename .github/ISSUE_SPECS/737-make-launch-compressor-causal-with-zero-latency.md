# Make the launch compressor causal with exact zero latency

## Problem, authority, and smallest closable product slice

The delivered compressor at main `a2e14c2e` always delays main audio by `Fs/50` samples (20 ms), including bypass and a lookahead setting of zero. It carries two `N+1` sample rings per channel and a staged processing path built around detector history. This is a poor default for the owner's utilitarian realtime mixing suite: the simplest academically grounded, low-CPU processor should have zero added latency wherever its product contract permits it.

The owner's explicit ruling is to remove compressor lookahead, its audio/detector delay buffers, and its staged processing path. Deliver one causal peak compressor with the existing banked SIMD arithmetic and seven existing controls. An advanced lookahead compressor is a separate future product decision. The earlier variable-lookahead-latency proposal is superseded; parameter-dependent latency infrastructure is neither needed nor authorized.

This issue closes when the causal compressor, its necessary descriptor/state changes, representative DSP/realtime/target evidence, affected current fixtures, and downstream graph/control contracts are delivered and remotely synchronized. It does not claim that the entire effect suite is zero latency. Generic harness work, extended research rates, a new listening campaign, and systematic optimization do not belong in this slice.

User-assigned workflow overrides the repository's default model routing: Astra XHIGH scopes, Luna XHIGH implements attempt 1, and Sol XHIGH adversarially verifies. Maximum five coherent implementation attempts, one adversarial verdict each; failed attempt five requires a preserved checkpoint and explicit rescope, never a disguised sixth retry. Root owns Git/GitHub, exact-path checkpoints, artifact pin qualification, and delivery. Use an isolated worktree; preserve the independent sparse-stem work.

## Frozen resulting compressor contract

- Keep effect ID `miso.compressor`, current contract pair `1.1`, live state-layout value `1`, and existing V1 wire/ABI identities under the prelaunch naming ruling. This issue explicitly amends the prelaunch descriptor and payload; it is not a compatible descriptor update. Derive a new descriptor identity from its changed bytes. Do not introduce V2 names or claim compatibility from the retained V1 label.
- Remove stable parameter ID 8 / descriptor index 7 (`lookahead`) completely. Never reuse ID 8 for another purpose. IDs 1–7, their order, domains, defaults, units, mappings, channel policy, legal automation, and 64-update smoothing remain unchanged. Preparation now takes exactly 14 ordered L/R initial records.
- Preserve Normal quality at exactly 44.1, 48, 88.2, and 96 kHz. Every quality and every prepared scalar/bank member reports `LatencySamples(0)`, enabled or bypassed. Preserve the conservative `TailSamples::Infinite` declaration for the gain envelope; a tail declaration is not signal delay.
- Preserve main input/output ports, optional sidechain, DualMono/Maximum/Average detector links, per-channel parameters/state, and resident gain-reduction observation.
- Preserve native homogeneous banking in every rack, current scalar fallback/tails, and W1/W4/W8 lane arithmetic. Removing staging does not remove SIMD or justify a second scalar implementation. The existing program key already incorporates latency; no program-key framework change is needed.
- Main and detector consume sample `n`, and gain computed for sample `n` applies to main sample `n`. There is no retained audio history, detector-history tap, cursor, delayed dry signal, staging buffer, or processing-quantum delay. Preserve the existing bounded ramping-prefix/idle-body specialization where useful; it is distinct from the deleted multi-pass staged path.
- Keep exact identity choices for valid audio: bypass, mix zero, and zero reduction with positive-zero makeup return current dry input bits; mix one returns wet. These selections continue advancing automation and gain state as currently contracted. Preserve mono-collapse admission, desymmetrization, silent-fixed-point proof, observation, and fault behavior, rewriting their state comparisons only to remove fields that no longer exist.

## DSP law and sound-quality boundary

Start with `dsp-research/dynamics.md`, `dsp-research/BIBLIOGRAPHY.md`, current `crates/compressor/src/{design,kernel}.rs`, and existing independent compressor tests. Historical #013/#046/#088/#475 records describe earlier contracts and remain historical evidence; this issue supersedes their lookahead/ring/staging requirements only. Current source and standing math contracts, rather than stale comments in old briefs, determine retained operation order.

For current detector sources `sL,sR`, preserve the existing magnitude/link laws: DualMono uses separate absolute magnitudes; Maximum shares their maximum; Average shares `0.5*abs(sL) + 0.5*abs(sR)` in that order. An unconnected detector uses current main input; a connected detector uses the current routed sidechain, and a connected-but-missing buffer retains the current silent-detector behavior. Graph PDC remains responsible for alignment of routed sources.

With threshold `T`, ratio `R`, knee `W`, detector level `X`, and `d=X-T`, the retained mathematical target is zero below the knee, `(1/R-1)*d` above it, and `(1/R-1)*(d+W/2)^2/(2W)` inside it; hard knee takes its existing separate law. The detector amplitude floor remains `1e-8`, level clamp `[-160,24]` dB, and target reduction clamp `[-100,0]` dB. Preserve existing sealed `fast_level_db`, `gain_delta_db`, and `fast_gain_from_db` calls and operation ordering; this is not a new transcendental approximation or gain-computer rewrite.

Rate coefficient `c = 1-exp(-1/(0.001*time_ms*Fs))` stays designed in f64 and rounded once to f32 at preparation/reset/restore and affected parameter updates, including the existing bounded 64-frame automation redesign. Attack is selected strictly when target reduction is below the previous reduction; equality uses release. Preserve the existing `rms_follow`/Lane arithmetic and flush: `G = flush(Gprev + c*(target-Gprev))`. Applied amplitude is the existing approximation to `10^((G+makeup)/20)`, with the existing gain/mix arithmetic and exact identity selections. Only the audio/detector time indices change.

Times, ratio, knee, level bounds, coefficient stability limits, preparation rejection of nonfinite/negative-zero values, and the current block-boundary NaN/recovery policy remain in force. Remove ring-specific state clearing without inventing new per-sample sanitation. Retain independent channel recovery and current finite signed-zero/subnormal handling. Full reset restores seven prepared defaults and clears gain state; discontinuity reset clears gain state and retains current target-snapping semantics.

The change intentionally stops anticipating transients. Finite attack permits initial transient overshoot relative to threshold; this compressor is not a brickwall or true-peak limiter. Zero latency means zero added sample delay, not instantaneous attack, zero envelope memory, or zero host I/O latency. Do not claim superior sound or bit identity to the old default 5 ms lookahead sound.

Primary basis: Giannoulis, Massberg and Reiss, *Digital Dynamic Range Compressor Design—A Tutorial and Analysis*, JAES 60(6), 399–408 (2012), indexed by [AES](https://aes.org/publications/journal-online/?num=6&vol=60); existing repo `[REISS-COMP]` derivation/reference tests supply the equation baseline. The bibliography's former QMUL PDF URL redirects to the department homepage as of this scope, so it is not evidence of a newly reread PDF. [Steinberg's processor latency contract](https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html) supports explicit reported sample latency. Neither source mandates compressor lookahead; choosing the causal topology is the owner's product ruling.

## State, compatibility, and resources

Use empty common payload and 22 little-endian words per channel: word 0 is gain reduction; words `1+3i`, `2+3i`, `3+3i` are current, target, remaining for parameter index `i=0..6`. Each channel is exactly 88 bytes; total payload is 176 bytes at all four rates. Keep existing validation bounds and transactional validate-both-then-commit restore. Preserve the existing mid-ramp step re-derivation and endpoint timing; exact mid-ramp continuation was not previously promised and is not added here. Remove cursor/lookahead/ring fields and all corresponding restore/copy/equality logic. An old raw payload rejects by exact length rather than being truncated or reinterpreted.

Keep the existing conservative `scratch_fixed_bytes=64`, `scratch_bytes_per_frame=0` reservation to avoid an unrelated scratch-accounting cleanup; delete actual `Staged` allocations. State/resource tests assert new exact sizes, exact-cap success and one-byte-below rejection. Saved payload size is not total heap size: evidence must distinguish these. Structurally verify no audio/detector/staging heap allocation remains and that preparation/runtime memory is independent of sample rate, quantum, and source duration apart from established outer bank/control allocations.

Old canonical Effect State V1 envelopes bind the old descriptor digest. They must reject transactionally under the new current descriptor before preparation or publication; old raw payloads must reject in scalar and bank hooks. The existing migration registry intentionally requires identical parameter lists and latency (`docs/EFFECT_STATE_MIGRATION_V1.md`), so no old-to-causal migration edge is legal. Do not weaken that registry or add a special migration adapter in this issue. Document that an old snapshot cannot continue its former lookahead sound; explicit new-session preparation is required.

Old session/control data explicitly naming parameter ID 8 must receive the existing typed unknown-parameter/invalid-address rejection at the appropriate admission boundary, without silently dropping, clamping, aliasing, or acknowledging it. Preserve atomic admission, revision, and snapshot behavior. The session's native effect selector contains only an effect ID, not a historical descriptor digest: an older session that omitted lookahead is indistinguishable from a newly authored seven-parameter/default session. Document that such sessions resolve to this amended causal prelaunch compressor; do not claim all old sessions can be detected or rejected. No session-schema redesign is authorized. Preserve stable IDs 1–7 and demonstrate canonical snapshot roundtrip for a valid current session.

## Allowed implementation and ownership

Luna owns `crates/compressor/src/{lib,design,kernel,state,corpus}.rs`, its relevant tests/support and example compile repair, and `crates/dsp-reference/src/compressor.rs` plus direct call-site changes required by removal of `ReferenceCompressorParameters::lookahead_ms`. Delete staged-specific code and `tests/staged_idle.rs` when its unique still-relevant assertions have been preserved in ordinary partition/identity tests. Remove tap/ring-specific test-only counters and obsolete expectations; do not delete general DSP, reset, fault, mono, automation, or realtime evidence with them.

Necessary descendant fixture/assertion changes are in `crates/effect-compiler/tests/{symmetry_designed_words,native_session,observation_identity}.rs`, compressor-related portions of `crates/graph-compiler/src/lib.rs`, `crates/host-core/src/scalar_point_endpoint.rs`, `crates/host-core/tests/effect_observation.rs`, `crates/effect-package/tests/descriptor_v1_qualification.rs`, and `tools/audit/src/compressor.rs` where inspection finds real old parameter/descriptor/resource assumptions. Existing generic descriptor/state/control/PDC/latency framework code stays unchanged unless a narrowly demonstrated existing assumption makes a necessary contract test impossible; root must amend scope before expanding that boundary.

Root owns changed current descriptor/corpus/artifact pins and necessary current fixture or fixture-generator references, notably compressor observation sessions and console sessions under `fixtures/session/v1/`, console derivation/validation scripts, metadata assertions, and supported Wasm artifacts. Freeze and record every expected identity/PCM/resource delta before resealing. Preserve old numbered evidence and historical digest records. Affected current pins necessarily move because the actual sound, state, and descriptor changed; a digest alone is not the sound-quality oracle. No `.ll` captures, whole-suite redesign, new benchmark framework, or unrelated source cleanup.

## Discriminating acceptance gates

1. **Causal signal and metadata.** At every launch rate, test scalar and supported bank preparation with exact zero latency. An impulse in bypass, ratio-one/unity-makeup, and mix-zero configurations emerges at the same index (including sample zero), with no leading quantum pad. An active-compression step uses the first causal attack update on sample zero. Two finite input sequences with identical main/sidechain prefixes but different later suffixes produce identical output prefixes. Test main-driven and connected-sidechain cases, asymmetric channels, and all link modes.
2. **Retained DSP.** Update the independent f64 reference to a direct current-sample topology; it must continue owning its own equations and state, without importing production coefficient/gain code. Retain the existing `2e-5` representative PCM oracle bound and existing curve/envelope/automation bounds. Exercise real gain reduction, hard/soft knee, makeup/mix identities, attack/release steps, coefficient-domain endpoints, and live ramps. No tolerance broadening or old-lookahead-versus-new-causal null claim.
3. **Determinism and optimization seams.** Retain native W1/W4/W8 exact PCM and valid state-at-rest identity, representative partitions around 64-ramp and 128-quantum boundaries (including 1,63,64,65,127,128,129), scalar tails, mono collapse/desymmetrization with state changes, silent fixed point, bypass warming, reset, and per-channel fault isolation. Remove staged branches, not these gates. Rebase the existing four-case cross-target corpus mechanically onto causal processing and obtain pins from its scalar instantiation; native vector widths and real scalar-Wasm/simd128 execution must independently confirm them.
4. **Compatibility and bounded resources.** Show new descriptor identity differs, exact 14-record preparation, ID 8 rejection for preparation/session/control, old descriptor-envelope rejection, and old raw-length rejection. Failure leaves destination/session/revision/output sentinels unchanged as appropriate. Test current scalar↔bank state interchange, invalid final right-channel word rollback, one-byte-short state and resource limits, and sample-rate-independent payload dimensions. No migration registry relaxation.
5. **Prepared graph.** Extend existing real-compressor integration fixtures to show zero output latency for a compressor-only route, same-sample dry/send/sidechain alignment, enabled/bypass equality of latency, and bit-identical rack placement/banked-versus-per-node PCM with live compression from the first block. Include a parallel path containing an unchanged real delayed effect or an existing fixed-latency fixture so removing compressor delay does not disable PDC for other nodes. Preserve typed resource/admission/backpressure failures; an ack must never precede a dropped command.
6. **Realtime and target gates.** Installed-allocation-counter tests must reach active scalar/bank processing, a parameter ramp, silence/reentry, and the relevant mono path with zero render allocations/frees. Reuse the existing compressor realtime audit entry point and source policy gates; no logging, locks, I/O, syscalls, or new unbounded calls. Run locked compressor debug/release all-target tests and focused strict Clippy, affected descendant tests, formatting, and realtime/lane/workspace policies. Root runs proportional workspace and supported-target qualification on the frozen source. Record actual nonzero test counts and supported Wasm execution, rather than crediting an empty filter or a cross-build as execution.

## Bounded CPU and listening evidence

This product change removes the four main/detector history allocations and four staging scratch allocations per scalar instance or bank, together with their ring/tap/staging work. Report those structural changes and actual allocation evidence separately from saved payload sizes. Removing staging does not by itself prove a speedup: staging previously exposed independent work to the processor.

The initial suggestion to run an existing compressor-only console measurement is superseded. `tools/bench/src/console.rs` accepts no workload selector and runs every console workload plus unrelated measurement arms; the historical runner and floor contracts do not provide a current one-row Issue 737 invocation. Do not invoke or extend that campaign for this issue.

Authorize a single bounded repair of `crates/compressor/examples/lane_sample_timing.rs` for native descriptive qualification. Root owns the external recorder, untimed preflight, provenance, overwrite refusal, persistence, and final invocation. Luna makes this example repair after the core checkpoint; it must not prevent the core tranche from being checkpointed. Sol approves the final harness, validator, workload and candidate before timing. No new benchmark framework or production instrumentation is authorized.

**Frozen workload.** Keep 48,000 Hz, 128 frames per block, 512 warmup blocks, and two measured rounds of 4,096 blocks each, per arm. The single invocation runs one scalar arm and one current-supported-native-bank-width arm; each receives one warmup and exactly two measured rounds. Scalar width is one. Use the seven resulting compressor defaults, DualMono main-input detection, enabled processing, and no automation. Keep the existing `SplitMix64` seeds (`0x5EED_0001/0002` scalar L/R, `0x5EED_0003/0004` bank L/R) and amplitude multiplier `0.5`. These are deterministic repeated synthetic noise blocks, not a music-program or whole-session workload. Remove the retired lookahead spread and 16-initial-record assumption; prepare the current ordered 14-value table.

**Input and timing.** Generate immutable source planes and allocate mutable process buffers once outside measurement. Before every warmup or measured block, copy the immutable source into the process buffers outside the timed interval. Never feed processed output back into the next block. Construct the process-block wrapper before starting the clock. Time only the public `process`/`process_bank` call between clock boundaries and sum integer elapsed nanoseconds outside that interval. Allocation, buffer copying, validation, observation/black-box consumption, formatting and I/O remain outside it. Processor state continues through warmup and both rounds; absolute first-sample values increase monotonically. Clock calls belong to the harness, never the production render path or an armed render audit.

Keep output observably consumed after processing so optimization cannot discard work. Untimed checks must establish finite nonzero input and output and actual sustained compression for each channel of every scalar/bank member after warmup: with this fixed unity-makeup/full-wet stimulus, output energy must be positive and below the corresponding input energy. No silent or identity result qualifies; one active bank lane does not qualify all siblings. Check actual process reports for zero invalid-span/fault counts. Preserve enough activity results in output to show the measured workload passed these checks. Per-block timing includes uncorrected clock-boundary overhead; disclose it, do not subtract a guessed calibration, and do not call the result cycle-exact isolated DSP.

**Untimed preflight.** Add one bounded `--preflight` mode to the existing example; reject unknown/additional arguments before processing. This mode exercises the real frozen input/preparation/process/activity checks with at most 16 untimed blocks per arm, validates dimensions/count arithmetic and emits only preflight records. It must call no clock and execute neither the 512-block warmup nor either 4,096-block measured round. The measurement invocation takes no arguments. Root preflights the recorder separately using synthetic records and a dummy process, proving argument handling, exact schema/row counts, malformed/missing-row rejection, nonzero-exit propagation, output persistence, and refusal to overwrite before launching timed work. This is a narrow example and recorder check, not a generic benchmark test framework.

**Output and persistence.** Emit one record for each measured round of each arm, with a simple frozen machine-readable schema: arm, round (1 or 2), sample rate, frames, measured block count, bank width, integer elapsed nanoseconds, integer lane-sample denominator, nanoseconds per block and nanoseconds per lane-sample, and untimed activity-check results. The denominator is exactly `4096 * 128 * 2 * width`; units are one channel of one track at one sample. Retain both rounds; remove best-of selection and the scalar/bank speedup ratio. Root records source commit/tree, example and validator identity, exact command, compiler/profile/configuration, CPU/OS/backend, stdout, stderr and exit status in one fresh externally preserved namespace. Native scalar plus available bank produces exactly four measured records. If the platform has no supported bank, emit an explicit unavailable record and exactly two scalar measured records; do not fabricate a bank value.

The recorder must create its output namespace exclusively and refuse an existing destination, preserve raw/partial output even on failure, and propagate binary/parse/validation failure as a nonzero result. No success summary or accepted record is published unless the process exits zero and every required row validates. A shell pipeline must not hide an upstream failure. Run the reviewed measurement binary exactly once; a failed post-workload step does not authorize rerunning it.

**Interpretation and stop rule.** Report both measured native nanoseconds-per-lane-sample and nanoseconds-per-block rounds on the named workload/host, with the clock-overhead limit. The previous feedback-loop example is not a valid baseline. Do not claim before/after speedup, whole-session capacity, CPU-floor percentage, or a generalized low-CPU win. Quote utilization only against an already-existing named applicable release budget with matching units; otherwise state that no new numerical budget is introduced. Do not infer cycles from an assumed CPU frequency. This is a product topology change, not an effect-optimization loop, so a class-A floor recount and console-derived `isolated_cycles_per_lane_sample` are not required.

If the one bounded example correction does not pass untimed preflight, or the single timed invocation's recorder fails afterward, preserve all evidence and move remaining measurement repair to one stateless successor. Do not expand the compressor issue into console/floor/runner work or retry timed execution. Product closure may candidly record numerical CPU qualification pending that named successor; it cannot claim an unmeasured performance target achieved.

Preserve #046's existing listening record/handoff as historical evidence and state its actual completion status. Document the audible design tradeoff: causal attack instead of transient anticipation. Deterministic transient/level-step renders and independent reference agreement are objective evidence, not human listening. If matched-loudness blinded listening of the changed sound has not occurred, record it as pending in standing listening qualification work. Do not invent a listening verdict or sonic-superiority claim. Human scheduling and a second fixture corpus must not keep this minimum product slice open.

Scope amendment verdict: **Astra XHIGH PASS** for this bounded qualification plan. This replaces the original compile-only example restriction and console/cycle/floor measurement requirements. No benchmark has been run and no measurement or listening PASS is claimed. Apply at the next root-owned core checkpoint so the active implementation base does not move mid-tranche.

## Suite latency direction and separate follow-ups

Bounded current inventory at the inspected base, derived from each crate's quality descriptor:

| Effect | Declared latency | Decision after this issue |
|---|---:|---|
| Compressor | `Fs/50` → `0` | This issue removes lookahead and history |
| Gate/expander | `Fs/100` (10 ms) | Separate causal gate slice; preserve hold/hysteresis/envelope law |
| Multiband compressor | `Fs/50` (20 ms) | Separate no-lookahead slice; retain existing crossover topology and independently prove its reconstruction/phase behavior |
| True-peak limiter | `Fs/100 + 6` samples | Owner algorithm ruling required; preserve peak guarantee until explicitly changed |
| Antialiased soft clipper | `31` samples | Owner antialias/filter-quality ruling required; no zero-latency relabeling |
| Parametric EQ | `0` | Preserve existing causal design |
| Transient shaper | `0` | Preserve existing causal design |
| Delay | `0` processing latency | Musical delay/tail is intentional behavior and is not removed |
| De-esser, dynamic EQ | No delivered crate in this inventory | Future ordinary mixing briefs start from a causal, zero-added-latency design; any unavoidable/optional nonzero mode needs an explicit product/quality ruling |

Queue the gate and multiband product slices separately after duplicate/issue-boundary reconciliation; do not implement them in parallel with this launch-critical feature. Neither future split grants authority to replace the limiter's true-peak guarantee, remove antialiasing, change crossover topology, introduce implicit phase-linear filters, or announce a completely zero-latency suite. A future advanced lookahead compressor must be separately briefed as an optional capability with its own explicit latency, identity, state, quality, and CPU budget.

## Delivery and initial status

Root creates the matching numbered GitHub issue and local spec in one checkpoint, verifies number/title and current base, and records scope approval before Luna implementation. Each coherent compiling/focused-green tranche pauses for root's exact-path checkpoint and delivery-mode-appropriate push; do not stack more work onto an uncommitted tranche. After Sol PASS, freeze source for root's required artifact/CI qualification, synchronize evidence upstream, merge under current repository policy, verify corrected main qualification and GitHub closure, then remove only clean fully pushed completed worktrees.

Scope approved by Astra XHIGH and root for issue #737 at base `a2e14c2ec06855142ee7a921db9461b4b5896fbc`. Worktree `/home/bl/misofm/engine-causal-compressor`, branch `codex/causal-compressor`. Exact scoped ownership is listed above; root must checkpoint coherent tranches before more edits. Luna XHIGH implements; Sol XHIGH verifies. No implementation or qualification PASS yet.

## Attempt 1 core checkpoint

Luna XHIGH implemented the causal Lane-generic core, seven-parameter descriptor,
zero-latency quality rows, 22-word per-channel codec, and direct independent f64
reference. Audio/detector histories, cursors and staged storage/processing are
removed. Eight source/reference/oracle-support paths form this compiling checkpoint.
Locked compressor/dsp-reference checks and the lib test command passed; the
integration-test compilation exits 101 on stale retired-contract tests (the initial
Luna summary incorrectly reported zero; inspected receipts correct that claim).
The focused independent oracle passes with
worst deviations 5.737e-7 and 2.980e-7, below the unchanged 2e-5 bound.

This is NOT full implementation/review PASS: old contract tests still fail on
retired lookahead/fixed-delay/resource expectations; other current tests, corpus
pins, descendants, fixtures, compatibility gates and CPU example remain pending.
No artifact pin or timing run is authorized by this checkpoint. Sol XHIGH review
follows the completed contract/evidence tranche. The CPU amendment above is
Astra XHIGH approved and applies after this root checkpoint.

## Attempt 1 current-fixture checkpoint

Root removed parameter ID 8 only from compressor instances in seven current session
fixtures and passed each through the current Rust canonical writer/native preparation.
The changed compressor counts are 8/8/1/1/64/64/64. All other model fields compare
identically, except six legacy decimal spellings normalized by the canonical writer
from 1.8499999999999999 to 1.85 with identical prepared f32 bits. Field formatting is
canonicalized; this accounts for the larger standing-console textual diff. The
intended-placement and mono derivation scripts independently reproduce their new
fixture bytes exactly. Historical numbered evidence is untouched.

Fixture receipts: `/tmp/issue737-fixture-transition-final`; core build/oracle and
preserved stale-test failures: `/home/bl/issue737-luna-attempt1-core-receipts`.
Initial root fixture preflight used a hyphenated binary filename and then a too-strict
Python f64 spelling comparison; both failed before qualification credit and were
corrected to the actual `session_validator` binary and explicit unchanged f32 proof.
Current corpus/descriptor/artifact pins and descendant expectations remain pending.
CPU qualification follows the approved bounded example amendment, not a console run.


## Attempt 1 contract-test checkpoint

Luna XHIGH adapted the causal contract, identities, ramps, payloads, fault, mono,
partition and silent-fixed-point tests. The new causality test compares equal
prefixes with differing future main/sidechain suffixes. Actual scalar and supported
bank impulse preparation/render now covers all four launch rates. Named sealed
math-crossing annotations are restored and current source prose no longer claims
retired rings/taps/staging. No DSP tolerance was broadened.

Root independently ran locked compressor integration compilation and the compressor
test suite with only `the_corpus_matches_its_pins_at_every_width` explicitly skipped:
71 top-level tests passed, plus one repeated nested allocation subprocess test.
The lib target contains zero tests and is not credited as behavior evidence.
Focused strict Clippy (`--lib --tests`) and workspace formatting checks passed.
Commands/output are preserved at `/tmp/issue737-contract-checkpoint`; preliminary
Sol review is at `/home/bl/issue737-sol-attempt1-core-review`.

This remains an intermediate checkpoint, not final PASS. W1/W4/W8 serialized-state
coverage, root-qualified scalar corpus pins, descendant session/control/envelope/PDC
gates, bounded CPU example qualification, supported Wasm/artifact qualification,
and final Sol review/CI remain pending. Sol's live-tranche checklist must be rechecked
on coherent source; passing tests alone do not resolve its coverage questions.


## Current SDK metadata transition

Root generated the current parameter metadata from Rust and verified the sole
semantic delta is removal of compressor ID 8: every retained parameter record and
every other effect's metadata is identical. Existing SDK codegen changes only the
catalog; generated ABI/provenance are byte-identical. The existing TypeScript
negative test now explains that lookahead is absent. Candidate derivation and
semantic comparison are preserved at `/tmp/issue737-metadata-candidate`. This
updates current metadata only; historical evidence remains unchanged.


## Attempt 1 integration checkpoint

The internal test now proves serialized state agreement at W1/W4/W8. Native session
preparation explicitly rejects stable compressor parameter ID 8. Existing graph
compressor fixtures now assert zero prepared latency and live first-block bank/scalar
identity; affected symmetry, control and observation tests pass. Current mutation
documentation identifies retired ring/staging evidence. Required adaptation of
`crates/host-core/tests/scalar_point_endpoint.rs` is within scope alongside the
previously named production endpoint path.

Preserved commands at `/home/bl/issue737-luna-attempt1-downstream-receipts` record:
compressor lib 1 test; native session 5; symmetry 2; observation identity 3; descriptor
qualification 6; generic envelope 1; host scalar 10 plus resource subprocesses; host
observation 10 with 1 ignored; focused graph 1; strict Clippy for affected crates.
All final commands exit zero. Test adaptation failures and their corrections remain
candidly recorded in that handoff.

The opaque-parameter-handle rejection test is generic control evidence, NOT stable
compressor ID 8 evidence; root caught and removed that incorrect claim before this
checkpoint. Likewise the existing generic descriptor envelope test is not credited
as an actual old-compressor envelope test. Stable-ID control rejection, an actual
old-compressor descriptor-bound envelope, mixed delayed-path graph alignment, final
Sol review, corpus/artifact qualification and CPU evidence remain pending. Earlier
7117b5f2 coverage of all-rate impulses, raw malformed payload hooks, future-suffix
independence and retained mono-state differences remains in place.
