# 076 Accept WebDriver null navigation success and close AudioWorklet browser proof

## Outcome

Correct the narrow WebDriver protocol adapter defect that stopped Issue 075 before product execution,
then reseal and execute exactly once the unchanged representative AudioWorklet browser correctness
proof. Preserve the checkpoint candidate, Wasm/JS product, independent oracle, fixture and product
gates; this issue changes no engine or AudioWorklet architecture.

## Status, technical input and attempt budget

**FINAL SOL FAIL / STOPPED / RESCOPED; NO OVERALL PASS.** Checkpoint `1875c97` preserves the
command-aware WebDriver response correction and its green nonbrowser gates as technical input only.
The sole no-browser seal authorization was consumed, so Issue 076 cannot reseal or run a browser.
Issue 077 owns the omitted explicit runner identities and later browser proof.

## Frozen correction

The W3C WebDriver Navigate To algorithm returns success with data null. Make the request adapter
command-aware: accept HTTP 200 `{ "value": null }` only for protocol commands whose success contract
permits null, including navigation and session deletion. Do not blanket-accept absent/null values for
new-session, status, script, element or other commands that require typed data. Preserve the same
endpoint, browser selection, fixture, expected output, candidate lifecycle and no-clobber behavior.

Hermetic tests must prove valid navigation null is accepted; a missing `value`, malformed envelope,
protocol error, and null for a command requiring typed data are rejected; and the former generic-null
behavior fails a mutation. The correction may touch only the existing browser runner and its bounded
hermetic tests/checks plus Issue-076 evidence. It must not edit Rust, Wasm, JS/worklet product files,
fixture/expected bytes, Cargo files, accepted corpora or CI.

## Reseal and sole browser authorization

First rerun every nonexecuting Issue-075 product/artifact/oracle/fixture gate needed to bind an
unchanged product to the corrected runner. On a clean committed candidate, create a fresh seal at a
new Issue-076 path with exact candidate/source/lock/tool/runner/fixture/artifact hashes and all
counters zero. Preserve the Issue-075 seal and failed evidence; never overwrite or reuse them.

Only after Sol independently verifies that seal may one fresh no-retry browser correctness command
run. It must execute the exact Issue-075 scalar plus supported-simd128 fixture, two fresh contexts per
backend, and all frozen PCM/status/resource/memory/source/seek/disposal gates. No direct browser,
runner bypass, tuning, substitute fixture or retry is permitted. A runner or product failure is final.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Stable C ABI and host-fed planar PCM render
- Exact lock-free native source sanitation telemetry handoff
- Production SIMD builtin bank graph retention and reachability qualification
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Non-goals and evidence

No product/API/ABI/DSP/source/session change, broad browser matrix, demo, deployment, long run,
memory/GC/performance measurement, SAB/Atomics, benchmark, timing or listening. Issue 074 retains all
separable qualification breadth.

Record the exact W3C response rule, mutation results, old and new seal identities, candidate/tool/
fixture/artifact hashes, sole browser outcome, cleanup/no-clobber result and strict Terra/Sol verdicts.
Workload/benchmark/timed counters remain zero.

## Primary reference

- [W3C WebDriver — Navigate To](https://www.w3.org/TR/webdriver2/#navigate-to)

## Final Sol evidence and disposition

On clean candidate `1875c97`, the exact two-file correction, response mutations, focused Rust tests,
Clippy/rustdoc, policies, scalar/simd128 artifacts, object/opcode checks and independent raw-Wasm
oracle passed. The sole authorized no-browser seal was written to
`/tmp/issue-076-browser-seal.json`: 2,744 bytes, SHA-256
`fcaf7688feee1bd3ba07f6b0ddf18c5ca8f4b9188827f990c0a3497b0fc6d638`, with all counters zero.

Final review found that the seal explicitly hashes product/source, fixture, artifact, browser and
driver identities but omits the four runner/tool identities frozen above:

- `scripts/web-audioworklet-browser-correctness.py`
- `scripts/test-web-audioworklet.sh`
- `scripts/seal-web-audioworklet-browser-correctness.sh`
- `scripts/run-web-audioworklet-browser-correctness.sh`

The candidate commit indirectly binds tracked bytes but does not satisfy the separately frozen
explicit runner-hash evidence. No browser was authorized or run, and no product/audio result exists.
Final counters are `browser_correctness_invocations=0`, `workload_invocations=0`,
`benchmark_invocations=0`, and `timed_invocations=0`. Issue 077 consumes this checkpoint as technical
input, not as an accepted dependency.
