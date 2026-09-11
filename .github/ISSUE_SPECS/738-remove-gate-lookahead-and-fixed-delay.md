# Remove gate/expander lookahead and fixed 10 ms audio delay

## Authority, base and smallest closable slice

The owner authorizes #738 and then #739 sequentially, with Astra XHIGH scope, Luna XHIGH implementation and Sol XHIGH adversarial verification. This brief activates only #738 at synchronized main `d2aeb62d4ed2b1b875a2e03ef31cd89c988081e9`, after causal compressor #737. Keep main's subsequent canonical-mono work intact. Deliver the existing ordinary gate/expander with current-sample detection and exact zero added latency, its necessary compatibility/state/fixture changes, and representative DSP/realtime/target evidence. Hold and envelope memory remain; audio history does not.

Maximum five coherent implementation attempts, one adversarial verdict per attempt; attempt five failure stops for preserved evidence and rescope. Root owns exact-path checkpoints, Git/GitHub, current fixture/metadata/corpus pins, artifact qualification and delivery. Luna pauses whenever a coherent tranche compiles and focused gates pass; root commits it before another tranche starts. No parallel #739 implementation. No new effect modes, crossover/limiter changes, generalized state migration, benchmark framework, historical resealing, `.ll` captures or optimization campaign.

## Frozen resulting contract

Keep `miso.gate-expander`, contract pair `1.1`, state-layout value `1`, existing V1 wire/ABI names, ports, observations and link modes. This is an explicitly incompatible amendment of a prelaunch descriptor, whose changed bytes derive a new descriptor identity; the retained V1 label is not compatibility evidence.

Remove stable parameter ID 8 (`lookahead`, old descriptor index 7) permanently; never reuse it. Exactly seven continuous PerLane parameters remain in this order. Preparation requires exactly fourteen ordered interleaved Left/Right initial records, indices 0 through 6; Both, missing, extra, misordered or invalid initial values retain typed rejection.

| Stable ID | Name | Unit | Domain | Default | Mapping | Automation / smoothing |
|---|---|---|---|---|---|---|
| 1 | threshold | dB | -80..0 | -40 | Linear | Block Point / Linear 64 samples |
| 2 | ratio | ratio | 1..20 | 4 | Logarithmic | Block Point / Linear 64 samples |
| 3 | range | dB | 0..96 | 80 | Linear | Block Point / Linear 64 samples |
| 4 | hysteresis | dB | 0..24 | 6 | Linear | Block Point / Linear 64 samples |
| 5 | attack | ms | 0.1..50 | 1 | Logarithmic | None / None |
| 6 | hold | ms | 0..1000 | 100 | Linear | None / None |
| 7 | release | ms | 5..2000 | 100 | Logarithmic | None / None |

Normal quality remains supported at exactly 44.1, 48, 88.2 and 96 kHz. Every quality/prepared member reports `LatencySamples(0)` enabled and bypassed, `TailSamples::Finite(0)`, 64 fixed scratch bytes and zero per-frame scratch. Gain/hold memory does not synthesize a signal tail: zero current main input produces zero output. There is no mix parameter.

Keep optional `sidechain-in`, DualMono/Maximum/Average detection, independent L/R controls/state, resident gain-reduction observation and ordinary bank eligibility in every rack. Connected sidechains retain scalar fallback. Gate currently provides no mono-collapse hooks: preserve that conservative fallback; prove equal-input mono behavior without inventing a new collapse implementation. Keep one Lane-generic arithmetic body at W1/W4/W8, native compile-time backend selection, scalar tails and current ramping-prefix specialization.

## Exact causal DSP, timing and faults

Start with current `crates/gate-expander/src/{lib,kernel}.rs`, `crates/dsp-reference/src/gate_expander.rs` and `dsp-research/dynamics.md`. This brief supersedes historical #014/#048/#089 lookahead/ring requirements only; it does not import their historical execution plans or qualification matrices.

At sample `n`, load both original main channel words before writing either output. Detector source is those current main words or the current connected sidechain words. Remove main/detector rings, taps, cursors, masks, gather scratch and tap-access classifiers, and their state/restore/copy branches. There is no one-sample or one-quantum pad. Do not add per-sample sanitation or alter malformed-block/sidechain policy as part of this timing change.

Advance each moving parameter ramp first, retaining current precomputed step, exact final target on update 64, step clearing and endpoint counts. For own/partner detector magnitudes, retain DualMono `abs(own)`, Maximum `max(abs(own),abs(partner))`, and Average `0.5*abs(own) + 0.5*abs(partner)` in the existing order. Preserve sealed `fast_level_db`/`fast_gain_from_db` and the existing level floor `1e-8`, then min(24), max(-160) dB clamps; no coefficient/transcendental rewrite.

Let detector level be `X`, ramped threshold `T`, ratio `rho`, range `R`, hysteresis `H`, prior gain `G`, and prepared hold length `K = floor(f64(hold_ms)*Fs/1000 + 0.5)`. Preserve these transitions:

- A closed gate opens when `X >= T` and reloads hold to K.
- An open gate at `X >= T-H` reloads K every sample, including equality.
- An open gate below `T-H` with a positive countdown remains open and decrements it once. It closes only when the countdown was already zero. Thus K successive below-band samples remain open after a reload; the next closes. Retrigger/rearm restores the full count.
- The target is zero when newly open; otherwise `clamp((rho-1)*(X-T), -R, 0)`. Select attack strictly when target > prior G, release otherwise. Apply the existing unfused Lane operation `Gnew = flush(rate*(target-G)+G)`, then current main times `fast_gain_from_db(Gnew)`. Bypass or `Gnew == 0` selects the original current main word exactly, including signed zero; bypass still advances state.

Keep `effect_runtime::envelope::attack_release_coefficient` unchanged: `1 - clamp(math::expf(-1/(time_ms*0.001*Fs)),0,1)` with the current f32 evaluation order. This differs from #737's coefficient design and must not be replaced by its f64 route. Valid descriptor times produce bounded stable coefficients; hold is at most 96,000, exact as an f32 integer. Only recursive gain is flushed below magnitude `1e-20`; preserve existing normal/zero validation and negative-zero rejection for prepared parameters. Main nonfinite output or nonfinite gain causes same-block lane-and-channel-local zeroing/full-default recovery at the existing block boundary, with unchanged counters and unaffected peers. A detector clamp alone is not proof every sidechain NaN is reported; retain the actual boundary policy, not an invented stronger guarantee.

Preparation and both reset kinds seed gain=0, open=1, hold=K. Full reset restores the seven prepared defaults; discontinuity retains unsmoothed times and snaps the four ramps to current targets. With hold=0 and a first sample below `T-H`, that sample transitions closed and uses the first release update from G=0. To prove opening from closed state, explicitly establish negative gain first; the trigger sample then uses the first attack update on that same main sample. Finite attack intentionally loses transient anticipation; zero added delay does not mean instantaneous full opening.

Primary basis is the repo's `[REISS-COMP]` citation to Giannoulis, Massberg and Reiss, *Digital Dynamic Range Compressor Design—A Tutorial and Analysis*, JAES 60(6), 399–408 (2012), for detector/time-constant analysis, not this gate's exact hysteresis policy. That policy is the retained independently tested engine contract. The [AES index](https://aes.org/publications/journal-online/?num=6&vol=60) fetch timed out during this scope; no new PDF-reading claim is made. [Steinberg's latency contract](https://steinbergmedia.github.io/vst3_doc/vstinterfaces/classSteinberg_1_1Vst_1_1IAudioProcessor.html) reports actual processor delay in samples and was consulted. Causal gate selection is the owner's product ruling, not a source-mandated design.

## Payload, resources and compatibility

Freeze the existing common codec's two little-endian u32 header words as `[1,44]`: layout identity and total data words. Each channel is exactly 22 little-endian f32 words:

| Words | Meaning |
|---|---|
| 0, 1, 2 | gain dB, open flag, remaining hold |
| 3, 4, 5 | attack ms, hold ms, release ms |
| `6+4*i .. 9+4*i`, i=0..3 | ramp current, target, step, remaining in retained ID order |

Common=8 bytes; left=88; right=88; total=184 at every launch rate, sidechain shape and width. No audio/detector history remains. Preserve strict section/header lengths, field bounds, precomputed ramp-step continuation, coefficient rederivation and parse-both-before-commit restore. Invalid late right-channel data must leave both channels and other bank tracks unchanged. Do not equate 184 saved bytes with the prepared allocation size: inspect removal of the two/four ring allocations separately. Remaining effect-owned preparation memory is independent of rate, quantum and stem duration apart from established outer control allocations.

Old V1 raw sections were common=8 and left/right=(23+2*N)*4 with N=Fs/100: 3,620 / 3,932 / 7,148 / 7,772 bytes at the four rates. They must reject by exact length at scalar and bank restore hooks, never truncate or reinterpret. Build one actual old-gate descriptor-bound envelope using the old eighth parameter and quality resources; verify it against the old binding first and require Descriptor rejection under the new binding. Reuse #737's test pattern in `descriptor_v1_qualification.rs`; a generic mismatched envelope is insufficient. Existing migration requirements forbid changing parameter list/latency, so no legal old-to-causal edge is added.

Explicit stable ID 8 in sessions/control receives existing typed unknown-parameter/invalid-address rejection before prepared publication or mutation admission. Preserve session/revision/snapshot and queue atomicity; no ack precedes a dropped command. Reuse #737's real stable-ID control lookup/transaction test with an actual gate address, not only an invalid opaque handle. The native session selector contains no historical descriptor digest: an old session omitting ID 8 is indistinguishable from a new seven-parameter/default session and resolves to this amended causal gate. Document that limitation; no schema redesign.

## Owned paths and early adaptation

Luna owns `crates/gate-expander/src/{lib,kernel,corpus}.rs`, its tests/support, and the independent gate reference and its direct callers. In the first core/contract tranches, replace obsolete tap classifiers, lookahead diversity, ring-wrap/restore and delayed-fault tests with their surviving causal/state/identity assertions. Update live source/test documentation at the same time: lib's allocation and obsolete “layout 2” prose, kernel's ring/gather and signed-zero explanations, reference dry/hold comments, corpus descriptions, and `tests/MUTATIONS.md`'s current contract. Historical mutation transcripts remain historical.

Necessary descendant edits are gate-related tests in `crates/effect-compiler/tests/{native_session,observation_identity,parameter_lattice}.rs`, `crates/effect-package/tests/descriptor_v1_qualification.rs`, `crates/graph-compiler/src/lib.rs`, and the existing host stable-ID transaction test surface used by #737. The graph's `accepted_gate_expander_graph_fixture` reuses the current compressor fixture with a selector change; preserve its valid seven-control assumptions and update actual 480-sample assertions/non-silence text. Reuse #737's mixed delayed-path PDC fixture for the gate rather than adding graph infrastructure. Update current ring descriptions in `tools/audit/src/gate_expander.rs` and `tools/wasm-gate-corpus/src/lib.rs`; preserve audit behavior and unrelated corpus cases. A small gate-specific current-contract paragraph in `dsp-research/dynamics.md` is sufficient research maintenance.

Root owns canonical removal of gate ID 8 from `fixtures/session/v1/observation-frame-shape.json` (the identified current session instance), current metadata regeneration via `sdk/codegen/assets.mjs` and `generate.mjs`, SDK generated catalog and focused rejection coverage, and corpus pins/artifacts. Current controllable row count becomes 65→64, three-point lattice coverage 195→192; retain all other row semantics. Existing descriptor fixtures are generic and do not need blanket resealing. Mechanically remove delay/tap plumbing from the existing six-case gate corpus while retaining its eight lanes, 1,024 frames, signals, remaining parameters, link cases, burst/subnormal/ramp coverage and non-vacuity checks. Root derives changed scalar pins only after oracle review and independently confirms vector and Wasm agreement. Supported Wasm artifact changes use the existing build/qualification flow; leave historical numbered artifacts and recorded hashes untouched.

## Minimum discriminating gates

1. **Causal DSP/resources:** all four launch rates report zero and exact 184-byte state, exact-cap preparation succeeds and one-byte-below state/scratch limits refuse. Same-index impulse including sample zero in bypass, ratio-one and range-zero configurations; active first-sample closing, explicit closed-state opening/release/retrigger and exact hold/rearm/equality counts. Current main and connected-sidechain causality with equal prefixes/different future suffixes; asymmetric link cases. No lookahead-based silence preroll may stand in for a closed-state setup.
2. **Retained arithmetic/state:** adapt the existing independent f64 oracle, preserving its 0.02 dB bound, reference independence and decision-margin checks. Retain exact W1/W4/W8 PCM and serialized continuation, active 64-sample automation, partitions straddling 63/64/65 and 127/128/129, bypass warming, signed zero, equal-input mono and asymmetric dual-mono, both resets, malformed final-right-word rollback, current scalar↔bank state interchange and per-lane/channel main/gain fault recovery. Internal W4 Lane evidence is not a claim the x86 factory admits W4.
3. **Admission/integration:** actual retired stable ID 8 initial/session/control rejection, old raw and old-gate-bound-envelope rejection, current canonical snapshot roundtrip. Real prepared graph confirms zero gate contribution, first-block active scalar/bank identity and unchanged rack placement/fallback; one existing mixed delayed route proves other nodes' PDC remains effective in enabled and bypass configurations.
4. **Realtime/targets:** reuse the installed-counter conformance tests and existing `audit` gate-expander entry point, including active scalar/available bank, connected sidechain, ramps and fault/reset coverage from focused tests. Zero render allocations/frees/locks/logging/I/O/syscalls; no new unbounded calls. Run locked gate debug/release tests, gate-reference tests, affected descendant tests, focused strict Clippy, formatting and existing lane/realtime/workspace policies. Root qualifies native corpus plus actual supported scalar-Wasm/simd128 execution and current browser/SDK artifacts; cross-builds and empty filters do not count as execution. Keep test counts and raw results, without adding a target framework or expanded fixture matrix.

## CPU, listening and delivery

Scope inspection found no gate selector/workload in the standing console `Workload` table, its fixture family, or `tools/bench/src/console.rs`; the runner is a multi-workload console campaign. Its preflight exists, but does not create a valid active gate measurement. The 100,000-block gate realtime audit has active input but is an allocation audit, not a console timing record. Therefore `timed_benchmark_invocations=0` for #738 and descriptive causal-gate CPU is pending **#746, “Qualify causal gate/expander CPU cost on an active-input workload”**. Do not repair/extend runners here or infer speedup/cycles from removed rings. That successor must freeze one active workload and validator, prove untimed argument/schema/persistence/exit/overwrite handling, then permit one root-owned invocation with exactly one warmup and two measured rounds, no timing retries or tuning. No numerical CPU release claim is made by #738.

No completed gate listening record was identified in `dsp-research/listening/`; #047's historical handoff requirements are not performed listening. Changed causal opening/listening stays explicitly pending in #26. Reference agreement and synthetic transient renders are objective evidence, not a human PASS or sound-superiority claim.

Root synchronizes this stateless local spec with existing open GH #738/title before implementation, keeps bounded CPU successor #746 synchronized, and records coherent checkpoint evidence. After Sol's singular final PASS on the frozen candidate, run required qualification on the exact PR head, merge through current policy, verify required qualification on the exact resulting main commit, synchronize evidence and close/verify GH #738. Only then report it delivered and begin active #739 implementation. Remove the completed clean, fully pushed worktree with preserved evidence; retain branches/history and any unique work.

Scope verdict: **Astra XHIGH PASS** at `d2aeb62d4ed2b1b875a2e03ef31cd89c988081e9`. No implementation, timing, listening or qualification PASS is claimed by this brief.


## Attempt 1 core checkpoint

Luna XHIGH supplied the causal kernel/runtime, independent reference and corpus
plumbing, plus seven-parameter support/contract scaffolding. Both current main
words are loaded before either output write; no ring/tap/cursor/classifier remains.
The descriptor and codec expose zero latency and common8+left88+right88 bytes.
`cargo check -p gate-expander -p dsp-reference` and three library tests pass; this
is a compiling core checkpoint, not final qualification. Old integration files
still contain retired delay/lookahead assumptions and await the next tranche.
Root preserved `/tmp/issue738-luna-tranche1.md` and restored only incidental
Cargo.lock dependency-order normalization. No corpus/artifact pin, metadata,
benchmark, or historical evidence changed. Oracle/current tests, admission/PDC,
root-owned fixture/metadata/pins, realtime/targets and final Sol/CI remain.


## Current session fixture checkpoint

Root removed exactly the gate ID8=2ms row from
`fixtures/session/v1/observation-frame-shape.json`. The actual current-engine
`session_validator validate --canonical` passes; the decoded canonical result
is otherwise identical to the candidate. Final SHA-256
`138c4eca45b7ee430013934323dfa606c686a7eff3f74dcfbb9cc17bd62f4878`.
Raw before/candidate/canonical and delta are in `/tmp/issue738-fixture`, validation
receipt in `/tmp/issue738-gates/canonical-fixture.*`. Synthetic metadata parser
fixtures are not live registry mirrors and do not require blanket resealing.
Current generated metadata remains root-owned and pending qualification.


## Attempt 1 focused test checkpoint

Luna XHIGH's focused tranche passes contract12, oracle3, identity4, state7,
conformance1 and gate library3 tests; the DSP-reference library passes25. The
independent oracle reports worst deviation `3.267e-4 dB` within the unchanged
0.02dB bound. Core/audit checks, formatting and diff checks pass. Root's existing
step-vocabulary policy also passes. Full receipts are in
`/tmp/issue738-luna-tranche2.md` and `/tmp/issue738-luna-tranche2-logs/`.

The six-case non-vacuity test passes. The sealed deterministic test intentionally
still fails at the old first scalar pin; no candidate pin is accepted yet.
Current-reference/docs/audit setup are updated, with historical mutation transcripts
preserved and labeled archived. Initial scaffold weaknesses from Sol's core review
were assigned to this tranche; independent post-checkpoint review must confirm
that surviving causal/state/hold/fault coverage is discriminatory. Actual stable-ID
session/control, old descriptor-bound state, graph/PDC, metadata/pins, realtime/
Wasm/browser and final CI remain. This is not a final attempt verdict.


## Current SDK metadata checkpoint

The actual Rust metadata export differs semantically only by removal of gate
parameter ID8. Root updates the current SDK asset/catalog and adds a typed
negative check for the retired gate lookahead name. All other metadata fields
remain identical; generated ABI/provenance and ABI-layout asset are byte-identical.
`check-sdk-generated.sh` and `check-sdk-types.sh` pass. External export/delta
are in `/tmp/issue738-metadata`, gate receipts in `/tmp/issue738-gates`.
No synthetic metadata self-test fixture or historical artifact is resealed.


## Attempt 1 integration checkpoint and remaining evidence repair

Luna XHIGH's bounded integration tranche passes native-session6, parameter-lattice10,
descriptor qualification8, graph compiler69 and host-web77 tests (2 ignored), plus
checks/formatting/diff checks. Actual gate stableID8 rejects before native publication
and in the host's atomic command batch; valid threshold plus retiredID8 admitszero
and preserves model/resources/next-block PCM, allowing only refusal-status reporting.
The reconstructed old gate descriptor/envelope verifies structurally under its old
binding and rejects under the current binding with Descriptor; this is not a claim
of replaying the old DSP implementation. Graph gates prove active first-block output
and zero gate contribution beside an unchanged486-sample limiter with preserved PDC
and bypass timing. Current lattice counts are64rows/192checks.
Receipts: `/tmp/issue738-luna-tranche3.md` and `/tmp/issue738-luna-tranche3-logs/`.

Sol independently passed the earlier oracle run but found Maximum/Average fixtures
always open and other focused tests insufficiently discriminatory. Corpus sealing
remains withheld. A separate isolated Luna XHIGH test-only repair at
`engine-causal-gate-738-tests` owns those already-frozen hold/state/fault/W4/identity
witnesses; integration owns disjoint paths. No final PASS or new production defect
is asserted. Root must integrate the repaired checkpoint and obtain Sol approval
before accepting current scalar corpus pins and final target/artifact qualification.


## Realtime audit preparation correction

The first actual audit invocation aborted at bank preparation: its local initial
value array still had the retired eighth lookahead value. No render marker or
render interval ran (zero BEGIN markers in the preserved trace). Luna removed
that final setup value only; locked audit check, formatting and diff checks pass.
Raw failed preparation is `/tmp/issue738-gates/realtime-audit.*` and
`/tmp/issue738-realtime/trace*`; repair receipt
`/tmp/issue738-luna-tiny-audit-repair.md`. Root must rebuild and rerun the realtime
audit after this checkpoint. This is not a timed benchmark and supplies no CPU
number; #746 remains the measurement successor.


## Focused repair and actual realtime qualification

Pushed test checkpoint 35ac102a integrates Luna XHIGH's isolated repair f66db7c8.
Contract20, oracle3, identity5, state7 and library/W4 4 pass, with formatting
and diff checks. The repaired fixtures require active nonclamped linked attenuation,
exact hold/retrigger and current-sidechain witnesses, all-rate caps and old-state
rejection, active-ramp reset, W4 continuation and atomic bank rollback/fault peers.
Receipt: `/tmp/issue738-luna-test-repair.md`. Sol review remains pending; this
checkpoint does not yet authorize corpus sealing or claim final delivery.

The corrected release realtime audit at b48a1d87 passes 100000 blocks of128frames
with an eight-lane bank and connected scalar path: allocations, deallocations,
locks, logs, file/network I/O and syscalls all zero. External trace validation
finds one render interval and zero violations. Receipts are
`/tmp/issue738-gates/realtime-audit-corrected.*` and
`/tmp/issue738-gates/realtime-trace-corrected.*`. The initial preparation failure
remains preserved. No timed benchmark ran and no CPU improvement is claimed.


## Current browser artifact candidate

Root's clean probe and ordinary pinned build at e39ab5cb reproduce Wasm
`3206cfa7657f420c21dcd17bc0963ccd3de38bd7bfa657e8d6a5054c8b9a79fc`.
Current metadata is `03736495d38fdd04012b39315431b14c1dcd5cf8d22ee81ecbc7b3f75e978fc8`
and matches the SDK asset byte-for-byte; the ABI and three JS/type companions
remain unchanged. Artifacts/hashes are external in `/tmp/issue738-artifact`;
probe/build receipts are in `/tmp/issue738-gates`. Browser qualification remains
pending. Descendant all-target tests (effect compiler/package, graph compiler and
host-web) pass. Sol permits scalar corpus derivation after the repaired oracle
and width/nonvacuity checks, while three focused test-strengthening items and
one fixture Clippy correction remain before final review. No production defect
has been found in this review checkpoint.


## Corpus and target qualification checkpoint

Sol XHIGH approved corpus derivation/sealing after independently checking active
nonclamped oracle attenuation in all three link modes: worst3.285e-4dB within
0.02dB. Root preserved derive-mode output (intentional final exit101 after all
six scalar/Simd4/Simd8 comparisons), sealed checkpoint06683913, and passed both
ordinary determinism tests including nonvacuity.

G5 passes native, actual scalar Wasm and simd128: each141cases/355comparisons,
zero mismatches, with the existing detector residency gate passing. Raw output
is `/tmp/issue738-wasm-gates/wasm-gates.jsonl`; root receipts remain in
`/tmp/issue738-gates`. Browser matrix checkpoint9ca6e940 records Chromium151.0.7922.34,
Firefox153.0 and WebKit26.5 PASS against candidate0668391387f2a1a9860cab278636bf48deb73fc8.
The first browser command rejected a short commit ID before qualification; the
canonical40-hex rerun passed. Both receipts remain preserved. Static artifact,
resource parity, hermetic host, SDK headless/package, matrix regeneration, lane/
realtime/workspace policies and strict gate/reference documentation all pass.

Final focused test repair, debug/release replay, strict Clippy, singular Sol
verdict and exact PR/main qualification remain before delivery.
