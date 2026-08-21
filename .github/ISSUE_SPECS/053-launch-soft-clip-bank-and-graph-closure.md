# 053 Launch soft-clip bank and graph closure

## Outcome

Close the stopped Issue 019 launch product by carrying its accepted fixed-2x cubic scalar checkpoint
into homogeneous W4/W8 banks, scalar tails, the launch registry/effect compiler, and one accepted
graph/PDC/resource vertical. Preserve the scalar algorithm and state without redesign.

## Context

Issue **Launch fixed-2x cubic soft-clip saturator** exhausted its Terra and Sol attempts without an
overall PASS. Checkpoint `e674d5e` is accepted technical input only: the Normal-only descriptor,
fixed 2x FIR/cubic scalar path, exact coefficient bits, 64-update automation, layout-1 state,
31-sample latency, finite tail 29, reset/recovery behavior, and focused scalar evidence remain
frozen. Its missing bank/registry/graph product gates cannot be added as a third Issue-019 attempt.

This stateless successor has exactly **two total attempts**: one Terra implementation/review and,
if needed, one bounded Sol correction/review. A second failure stops. Render remains allocation-
free, lock-free and bounded. `timed_benchmark_invocations=0`; no benchmark is authorized.

## Scope

- Implement homogeneous W4/W8 `miso.soft-clip` banks with independent dual-mono/per-track state,
  exact scalar operation order per vector lane, and scalar tails for every track count.
- Execute the one already-frozen representative alias row needed to support the product's
  “antialiased” claim; do not add a corpus or parameter matrix.
- Register the effect in the launch native registry/effect compiler and update only its exact
  dependency-policy allowlist and mutations.
- Retain one ten-track 48-kHz/128-frame graph fixture proving width-correct banks and tails,
  latency/tail/bypass/PDC, scalar-delegate PCM/state, and transactional post-bank capacity.
- Close the focused and clean nonbenchmark workspace/policy gates.

## Required public interfaces/contracts

The Issue-019 scalar descriptor, parameter IDs/domains/order, table and arithmetic, automation,
ports, latency/tail, snapshot bytes, reset/recovery rules and scalar output are immutable. The
effect is Normal-only, DualMono, has required dual-mono `main-in`/`main-out`, and has no sidechain.

Each bank track owns the exact scalar state: 676 bytes per lane, 1,352 bytes dual-mono, plus 24
bytes of retained reset defaults. Therefore exact effect-owned retained bytes are:

| width | retained effect bytes |
|---:|---:|
| W4 | 5,504 |
| W8 | 11,008 |

W4 uses the accepted Wasm-simd128/NEON width and W8 uses AVX2. Base and AVX2+FMA selections execute
the same frozen multiply-then-add graph with zero contractions. Every member is validated before a
legal unavailable backend may return `Ok(None)`; malformed requests never become fallback. No
padding track, shared lane state, compiled track ceiling, hidden allocation, or changed state
payload is permitted.

Bypass and mix-zero/unity-output preserve the 31-sample delayed dry bits while warming complete wet
state. Enabled metadata remains `LatencySamples(31)` and `TailSamples::Finite(29)`, with final
causal support at base sample 60. Width metadata, AoSoA histories, member buffers and graph-owned
scratch are included through the accepted checked post-bank graph accounting.

## Deliverables

- W4/W8 homogeneous bank plus scalar-tail behavior and representative direct parity/resource tests;
- the single frozen alias-claim result;
- registry/effect-compiler and exact policy integration;
- one ten-track graph/PDC/cap fixture; and
- final focused/workspace/policy evidence with an explicit PASS/FAIL verdict.

## Explicit non-goals

Scalar DSP/table/domain/state redesign; another nonlinear mode, factor or quality; general
oversampling/SRC; expanded transfer/alias/rate/drive/mix corpus; 10,000 or million-sample rows;
100,000-render audit; cross-target or instruction inspection; benchmark/preflight/timing;
optimization; audition or listening. Those broad qualification surfaces remain Issue 052,
**Launch saturator/clipper qualification, realtime audit, and benchmark**.

## Dependencies by exact issue title

- Launch fixed-2x cubic soft-clip saturator
- Native effect runtime contract and conformance
- AoSoA SIMD rack compiler and scalar/AVX2/WASM kernels
- Production SIMD builtin bank graph retention and reachability qualification
- Deterministic graph compiler, sends, submixes, sidechains, and PDC

The stopped Issue 019 contributes only its explicitly accepted scalar checkpoint, not an overall
PASS.

## Sol implementation brief

**READY FOR TERRA ATTEMPT 1 after local/remote Issue 053 synchronization.** The tracked
authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/053-launch-soft-clip-bank-and-graph-closure.md`. This checkpoint
authorizes no benchmark.

## Hazards/decisions

The bank is a fixed-width packing of the accepted effect-local 2x realization, not a reusable
oversampling framework. Vector lanes remain independent and use the scalar tap/polynomial order;
backend FMA capability must not change results. The representative alias row is retained because
without it the product cannot honestly claim measured antialiasing; every broader qualification
row is deferred.

## Acceptance gates with objective measurements

1. The accepted scalar descriptor, coefficient bits, output/state behavior and resource values are
   unchanged. The frozen 16,384-sample bin-3001, +18-dB-drive row records the 2x and naive-1x ratios
   and improves total nonfundamental/fundamental energy by at least 2.0 dB exactly as Issue 019
   specified.
2. Bank binding rejects wrong width/backend/count, malformed members, program/quality/port changes,
   invented sidechain and cap failure before unavailable fallback. Exact W4/W8 retained bytes pass;
   one byte below rejects transactionally without consuming ownership.
3. Available native bank PCM, complete per-track state and reports match scalar peers bit-exactly
   across consecutive blocks with representative per-track automation, bypass/identity warming,
   active restore, both resets, signed zero, sanitation and one injected lane-local recovery.
   Scalar tails remain ordered and isolated.
4. The ten-track fixture retains one W8 bank plus two scalar tails on W8, two W4 banks plus two
   tails on W4, or ten scalar instances otherwise. It proves stable membership, exact latency 31,
   tail 29/support 60, bypass/PDC/canonical stability, scalar-delegate PCM/state, corrected post-bank
   resource estimates and one-byte-below ownership return.
5. Focused soft-clip/core/effect-compiler/registry/graph tests, formatting, warning-denied Clippy,
   one locked workspace check/test/Clippy/rustdoc seal and applicable workspace/realtime/effect-
   runtime/rack/graph policies pass. No Issue-052, audit, target/object, benchmark, timing or
   listening command runs; `timed_benchmark_invocations=0`.

## Target matrix

Execute scalar and the available native W8 backend on the candidate host; compile and test the W4
source contract through focused checks. Cross-target and named-instruction qualification belongs
only to Issue 052.

## Required evidence

Accepted scalar checkpoint and candidate identities; exact W4/W8 resource rows; request mutation
and unavailable-fallback results; scalar/bank PCM/state/report and recovery/isolation rows; frozen
alias ratios; ten-track bank/tail/latency/tail/PDC/cap report; focused/final/policy outputs; attempt
count; strict Terra/final Sol verdict; and `timed_benchmark_invocations=0`.

## Terra attempt 1 evidence — product-bank checkpoint (FAIL)

- Product-bank code compiles, including W4/W8 binding, request validation-before-fallback and the
  width-specific retained rows (5,504 / 11,008 bytes).
- Focused `cargo test --locked -p miso-engine-soft-clip --lib` result: **FAIL**, 6 passed / 7
  total. `tests::available_w8_bank_matches_scalar_state_reports_and_lane_isolation` fails at
  `crates/miso-engine-soft-clip/src/lib.rs:1791`: the injected W8 interpolation-history fault does
  not set `recovered[0]`. This is likely an intermediate-visibility contract issue in the direct
  recovery probe; it was not repaired in this attempt.
- Remaining core/Clippy/final-policy gates were not run after the semantic failure. No alias,
  registry, graph, qualification, audit, target, or timing work was started.
- `timed_benchmark_invocations=0`. Terra verdict: **FAIL**; stop for Sol review.

## Sol attempt 2 — bounded recovery-parity correction checkpoint (partial)

- Base candidate `426efc6`. The Terra failure reproduced unchanged: an injected W8 interpolation
  `NaN` did not set lane zero's recovery result. Root cause was the core phase seam, not the frozen
  scalar DSP or test index: `PreparedSoftClipBankKernelV1::process_phase` returned only structural
  errors, so a nonfinite FIR accumulator/cubic intermediate was not observable by the effect when
  the current decimator output remained finite.
- The prepared core token now returns an allocation-free `u32` failed-lane mask. Scalar, W4
  Wasm/NEON, W8 AVX2 and the zero-contraction AVX2+FMA alias check and normalize the input, every
  FIR product/add, and every cubic multiply/divide/subtract rounding point. Nonfinite values set
  only their lane bit and become positive zero; finite subnormals become positive zero without a
  recovery bit. Only healthy lane cursors advance. Dispatch remains preparation-gated and performs
  no render allocation, feature detection or lock.
- The bank merges both high-rate phase masks into the existing per-track recovery flags. A failed
  lane emits its already captured delayed dry sample, clears/snaps only that lane in the same host
  sample and increments only its corresponding `ProcessReport`; all other channels/tracks continue.
  The descriptor, coefficient table, scalar implementation, state layout and exact 5,504/11,008
  retained-byte rows are unchanged.
- Executed proof now covers a portable scalar nonfinite/subnormal phase, an AVX2 interpolation plus
  decimation two-lane fault mask with all healthy cursors isolated, and the real W8 bank report.
  The bank's recovered left lane matches scalar delayed-dry output and complete serialized state;
  every other left lane and all right lanes match an uninterrupted healthy scalar lane.
- Focused PASS: `cargo fmt --all -- --check`; `cargo test --locked -p miso-engine-core --lib`
  (31 passed); `cargo test --locked -p miso-engine-soft-clip --lib` (7 passed); and `cargo clippy
  --locked -p miso-engine-core -p miso-engine-soft-clip --all-targets -- -D warnings`.
- This is the first bounded Sol correction checkpoint, not overall Issue-053 PASS. The frozen alias
  row, registry/effect-compiler, graph/PDC/cap and final workspace/policy gates were not started.
  Issue-052 qualification remains untouched; `timed_benchmark_invocations=0`.

## Sol attempt 2 — frozen representative alias checkpoint (partial)

- Base candidate `6b5f5d4`; production DSP, coefficient table, domains, state, resources and
  tolerances are unchanged. Added only the frozen `N=16,384`, bin-3001, unit-sine, `+18 dB` drive,
  `0 dB` output, mix-one row after three complete warm periods.
- The production scalar output is measured with a rectangular DFT energy identity: direct DC and
  fundamental-pair evaluation plus Parseval includes every other positive and negative bin as
  nonfundamental energy. The baseline calls the independent reference crate's f64 memoryless cubic
  at naive 1x; it imports no production table or implementation.
- Deterministic serialized result:
  `fixed_2x_nonfundamental_ratio_db=-17.090501510225`,
  `naive_1x_nonfundamental_ratio_db=-7.291819669285`,
  `improvement_db=9.798681840940`. The frozen `>=2.0 dB` gate therefore passes without a retry or
  adjustment.
- Focused PASS: exact test with `--nocapture`; `cargo test --locked -p miso-engine-soft-clip --lib`
  (8 passed); `cargo test --locked -p miso-engine-dsp-reference` (9 passed, 1 unrelated ignored);
  `cargo clippy --locked -p miso-engine-soft-clip -p miso-engine-dsp-reference --all-targets -- -D
  warnings`; and `cargo fmt --all -- --check`.
- This remains a partial Issue-053 checkpoint, not overall PASS. Registry/effect-compiler,
  ten-track graph/PDC/cap and final workspace/policy closure were not started. No Issue-052,
  benchmark, timing or listening work ran; `timed_benchmark_invocations=0`.

## Sol attempt 2 — registry and ten-track graph checkpoint (final-ready partial)

- Base candidate `e2da972`. `miso.soft-clip` is now the sixth injected launch-native factory and an
  exact direct dependency of the effect compiler. Baseline policy plus arbitrary-extra,
  missing-soft-clip and substituted-soft-clip mutations pass; no registry singleton or render-time
  lookup was introduced.
- One accepted 48-kHz/q128 ten-track fixture uses homogeneous Normal, DualMono, no-sidechain
  soft-clip programs with legal per-track/lane drive differences. The candidate host retained one
  W8 bank over `eq0..eq7` plus ordered scalar tails `eq8,eq9`; the scalar-delegate artifact retained
  no bank. Consecutive-block bank/tail PCM is bit-exact to ten scalar delegates, exercising carried
  state; the earlier direct bank checkpoint remains the complete byte-state/report parity proof.
- Every prepared enabled and bypassed effect reports latency 31 and finite tail 29. Enabled impulse
  output has absolute peak 31, nonzero final support sample 60 and exact zero thereafter; the next
  block remains zero. Enabled, scalar-delegate and bypass artifacts retain identical sequential
  schedule, route timings, inserted delays and canonical graph bytes, with every route arriving at
  sample 31 and zero compensation.
- Corrected post-bank accounting is non-vacuous: the W8 graph independently derives and matches one
  retained bank, 16,384 AoSoA scratch bytes, 8,192 runtime member-buffer bytes and the exact checked
  bank/member metadata delta. Its audio-sample, graph-metadata, incremental-plan and session-plus-
  plan deltas match the scalar artifact. A cap at `incremental_plan_bytes - 1` rejects with
  `graph.resource.limit`, publishes no graph and returns all ten prepared entries/session tracks.
- Focused PASS: exact soft-clip graph test; locked effect-compiler tests (4 passed), graph-compiler
  library tests (20 passed) and soft-clip library tests (8 passed); warning-denied all-target Clippy
  for soft-clip/effect-compiler/graph-compiler; `cargo fmt --all -- --check`; effect-runtime baseline
  and mutations; rack baseline and mutations; and graph policy.
- This checkpoint is final-product-ready but not yet an overall Issue-053 PASS: the separately
  authorized clean workspace check/test/Clippy/rustdoc seal has not run. No Issue-052, audit,
  cross-target, instruction, benchmark, timing or listening work ran;
  `timed_benchmark_invocations=0`.

## Sol attempt 2 — final nonbenchmark product seal

- Final candidate `9a382c1` preserves the accepted Issue-019 scalar checkpoint and all three
  committed Issue-053 correction checkpoints above. No production, fixture, policy or
  qualification file changed during this seal.
- `cargo fmt --all -- --check`: PASS. `cargo check --locked --workspace --all-targets
  --all-features` and `cargo test --locked --workspace --all-targets --all-features`: PASS,
  including the eight soft-clip product tests and the ten-track graph closure fixture.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS.
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --all-features --no-deps`: PASS.
- Workspace, realtime, effect-runtime and rack baseline plus mutation suites PASS when invoked
  through `bash`; graph baseline policy PASS. The graph policy has no separate mutation script.
- The Git-free static seal found no conflict markers or trailing whitespace in the Issue-053
  product paths, no `.orig`, `.rej`, `.tmp` or `.profraw` artifacts outside excluded build/VCS
  directories, and valid shell syntax for every invoked policy script.
- No Issue-052 qualification, functional audit main, cross-target, object inspection, benchmark,
  timing or listening command ran. `timed_benchmark_invocations=0`.

**Final Sol verdict: PASS.** Issue 053 closes the fixed-2x soft-clip bank, measured alias claim,
launch registry/effect compiler and ten-track graph/PDC/resource product vertical in the second and
final authorized attempt. Issue 052 remains the sole owner of deferred broad qualification,
realtime audit, target/instruction, benchmark and listening work.
