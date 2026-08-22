# Sol implementation brief — issue 076 WebDriver null navigation and AudioWorklet browser proof

## Decision

**FINAL SOL FAIL / STOPPED / RESCOPED; NO OVERALL PASS.** Checkpoint `1875c97` preserves the
command-aware response correction and green nonbrowser evidence. Its sole seal, SHA-256
`fcaf7688feee1bd3ba07f6b0ddf18c5ca8f4b9188827f990c0a3497b0fc6d638`, omitted explicit hashes for
the runner, test, seal and run scripts required by the brief. No browser ran; all invocation counters
remain zero. Do not reseal or continue Issue 076. Deferred Issue 077 owns that exact evidence defect.

## Literal implementation surface

- Replace the generic non-DELETE `value is None` rejection with an explicit command/result contract.
  Navigate To's HTTP 200 `{ "value": null }` and session deletion are valid; commands requiring a
  typed value still reject null, missing `value`, malformed envelopes and protocol errors.
- Add hermetic transport tests and a mutation that reinstates the generic rejection. Do not start a
  browser in these tests.
- Freeze every Issue-075 Rust/Wasm/JS/fixture/oracle identity. Only the browser runner and its bounded
  tests/checks may change.
- Run the prior nonexecuting seal gates, create a fresh Issue-076 no-clobber seal on a clean commit,
  then hand the exact command and identities to Sol.

## One browser authorization

Sol may authorize one runner invocation only after independently validating the fresh seal. It uses
the unchanged scalar/simd128, two-context, 48-kHz representative fixture and all frozen PCM/status/
resource/memory/lifecycle assertions. No retry or direct browser invocation. Any failure stops.
Issue 074 owns all matrix, long-run, deployment and performance breadth.
