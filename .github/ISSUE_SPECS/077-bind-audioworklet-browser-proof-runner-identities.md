# 077 Bind AudioWorklet browser proof runner identities

## Outcome

Close the sole evidence defect exposed by stopped Issue 076: explicitly bind every script that can
seal, validate or execute the representative AudioWorklet browser proof, then create one fresh seal
and seek one later no-retry browser authorization. Preserve all product, fixture, oracle, artifact and
browser-test semantics from checkpoint `1875c97`.

## Status, technical input and anti-stall disposition

**SOL-BRIEFED / DEFERRED / NOT STARTED.** Stopped Issue-076 checkpoint `1875c97` and seal SHA-256
`fcaf7688feee1bd3ba07f6b0ddf18c5ca8f4b9188827f990c0a3497b0fc6d638` are technical input, not an
accepted dependency. Three consecutive AudioWorklet rounds—Issues 024, 075 and 076—have stopped.
Per the anti-stall delivery rule, orchestration moves to another dependency-ready feature before
authorizing this work. When resumed, permit exactly one Terra attempt and one bounded Sol correction;
a second failure stops.

Issue-077 counters begin at `seal_invocations=0`, `browser_correctness_invocations=0`,
`workload_invocations=0`, `benchmark_invocations=0`, and `timed_invocations=0`.

## Frozen correction

Add an exact address-free `runnerSha256` record with these four and only these path/hash pairs:

- `scripts/web-audioworklet-browser-correctness.py`
- `scripts/test-web-audioworklet.sh`
- `scripts/seal-web-audioworklet-browser-correctness.sh`
- `scripts/run-web-audioworklet-browser-correctness.sh`

Both seal creation and the pre/post-browser equality checks must recompute this complete map. Keep the
clean candidate commit check as a separate invariant. Hermetic nonbrowser coverage must mutate each
of the four paths independently and prove seal validation rejects it before any browser launch, then
prove the exact clean map accepts. Missing/extra keys, symlinks and changed bytes reject.

No product, Rust, Wasm, JS/worklet, fixture, expected oracle, Cargo, accepted-corpus, browser identity
or browser assertion may change. Any need to change those surfaces is a STOP.

## Seal and browser lifecycle

After the full frozen Issue-076 nonbrowser gates pass on a clean committed candidate, Sol may
authorize exactly one fresh no-browser seal at a new Issue-077 path. The stopped Issue-075/076 seals
remain preserved and are never overwritten or accepted as the new seal. If the new seal passes an
independent identity audit, Sol may separately authorize exactly one no-retry execution of the
unchanged representative browser runner. Direct browser execution, retry, tuning or fixture
substitution is forbidden.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Stable C ABI and host-fed planar PCM render
- Exact lock-free native source sanitation telemetry handoff
- Production SIMD builtin bank graph retention and reachability qualification
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Evidence and non-goals

Record the four exact hashes and mutation results; candidate/source/lock/tool/fixture/artifact hashes;
old and new seal identities; no-clobber lifecycle; the sole later browser result; and strict Terra/Sol
verdicts. Browser/workload/benchmark/timed counters remain zero until separately authorized.

No architecture, product, fixture, oracle, broad browser matrix, long run, deployment, performance,
benchmark, timing or listening work. Issue 074 retains all separable qualification breadth.
