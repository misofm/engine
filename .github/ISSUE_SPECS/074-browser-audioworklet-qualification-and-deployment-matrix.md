# 074 Browser AudioWorklet qualification and deployment matrix

## Outcome

Qualify the accepted **Close AudioWorklet lifecycle and backend identity** product across the launch
browser and deployment matrix without changing its Wasm ABI, JavaScript API, engine behavior or
artifacts.

## Status and attempt budget

Stateless successor; begins only after Issue 075 passes. Stopped Issue 024 and checkpoint `ba7ffc6`
are historical technical input only, not accepted dependencies. Permit one Terra attempt and one
bounded Sol correction. A second failure stops. Benchmark/timed invocation count starts at zero; any future
descriptive measurement must be frozen and authorized here before it runs.

## Scope

- Add a checked-in local demo and deployment guide for secure-context AudioWorklet loading, MIME and
  cache behavior; no COOP/COEP or shared-memory claim is required.
- Run pinned current Chromium, Firefox and WebKit desktop rows plus supported mobile-browser/device
  rows for scalar selection, simd128 selection/fallback, source/seek/backpressure, deterministic PCM,
  sticky error/dispose and context suspend/resume.
- Run the frozen one-million-quantum offline stability row and ten-minute live worklet row with
  unchanged Wasm memory, bounded host memory, no unexpected GC/render errors and preserved PCM/status
  digests.
- Record bundle sizes and one separately frozen descriptive browser performance workload. If timed,
  it may run exactly once with one warmup and two measured rounds, no threshold or retry.

## Explicit non-goals

Product API/ABI or DSP changes, SAB/Atomics/threads, plan swap, browser multicore, decoder/SRC,
network PCM, third-party Wasm, native/mobile-native embedding, benchmark tuning or subjective audio
claims. A product defect discovered here fails and returns to a new bounded issue.

## Dependencies by exact issue title

- Close AudioWorklet lifecycle and backend identity

## Acceptance and evidence

All named browser/version/device rows load the sealed scalar/SIMD artifacts and reproduce the Issue-
075 fixture within its frozen gate. The offline/live rows preserve memory and report zero render/
schema errors. Deployment/demo files pass static and local-server smoke. Record exact candidate,
source/lock/artifact/fixture hashes; browser/OS/device versions; feature selection; PCM/status/memory/
GC results; strict Terra/Sol verdicts; and exact browser workload/timed invocation counts.

This issue gates only end-to-end release qualification.
