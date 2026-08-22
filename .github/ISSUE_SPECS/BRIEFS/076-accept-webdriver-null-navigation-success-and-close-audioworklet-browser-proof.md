# Sol implementation brief — issue 076 WebDriver null navigation and AudioWorklet browser proof

## Decision

**READY FOR TERRA ATTEMPT 1.** Consume stopped Issue-075 checkpoint `a0c46af`, its unchanged fixture
and preserved failed seal as technical input only. Use one Terra attempt plus one bounded Sol
correction. Correct only the command-specific WebDriver null-success handling, reseal on a clean
candidate, and execute the unchanged representative browser proof at most once after explicit Sol
authorization. Issue-076 browser/workload/benchmark/timed counters begin at zero.

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
