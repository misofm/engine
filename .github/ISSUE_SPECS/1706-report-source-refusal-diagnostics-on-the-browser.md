# Report source refusal diagnostics on the browser as on the C ABI

Filed by root from the batch 16 #1293 review (MINOR 5). It is the "source submit
diagnostics" one-host difference that decision 17 C13 lists
(`docs/rulings/one-control-abstraction-2026-10-13.md`, C13).

## Product outcome

A browser host that is refused a source call (submit, seek, anchored seek, session seek or seek
report) reads the same typed diagnostic as a C host: for example `source.id.unknown`,
`source.seek.anchor_unaligned`, `source.generation.stale`, `source.seek.backpressure`,
`source.report.busy` or `capi.source.epoch`'s browser twin. One core function owns the mapping from
a `SourceFailure` to its result kind and diagnostic; both hosts call it.

## Context (anchors on `5567f7487`)

- The C ABI maps `SourceFailure` to `(result code, diagnostic bytes)` in
  `crates/capi/src/runtime/error.rs:121-143` and writes the diagnostic to
  `miso_engine_v1_last_error`.
- The browser maps the same failure in `hosts/host-web/src/lib.rs:5338-5356`
  (`source_result`, `source_failure_result`) to a result code only, and drops the core's
  diagnostic. The result-code rule is written twice, once per host: logic in two adapters.
- The core's text is `SourceControlError::diagnostic` (`crates/host-core/src/source.rs`).

## Decisions to make at filing

- D1. One core mapping, in `crates/control-plane` beside `SourceFailure` (or in host-core):
  `SourceFailure::report() -> (SourceResultKind, &'static str)` (kinds: ok-refusal invalid
  argument, backpressure, internal). capi and host-web map the kind to their result codes and
  write the text to their diagnostic buffers; neither holds the rule.
- D2. The browser writes the diagnostic where `AudioWorkletEngineHost::diagnostic` is read today,
  for every source export, and the SDK surfaces it as it surfaces other refusals.
- D3. Parity row in `hosts/host-web/tests/control_parity.rs`: each refusal kind of each source call
  gives the same result kind and the same diagnostic on both hosts.

## Dependencies

- None blocking. #1644 (browser SDK `seek`) benefits from it.

## Decision record (root, 2026-10-24; scope note wins)

- D1 as filed, named `SourceRefusalKind { InvalidArgument, Backpressure, Internal }` with
  `SourceFailure::kind()` and `SourceFailure::diagnostic()` in `crates/control-plane` beside
  `SourceFailure`. capi's `SourceFailureReport` and the browser's `refuse_source` keep only the
  3-arm kind-to-code map.
- D2 (scope D2): the epoch failure reads `source.epoch` on both hosts; the C ABI's former
  `capi.source.epoch` is a prelaunch string change (root-accepted), header and docs follow.
- D3 (scope D3, replaces D2 above): `AudioWorkletEngineHost::refuse_source` writes the text into
  the session diagnostic with `ControlResponse::Diagnostic`, read by `diagnostic()` and
  `miso_engine_web_v1_control_response_ptr/_bytes`. The JS message shapes and the SDK surface are
  #1644's (non-goal here).
- D4 (scope D4): `SourceControlError::InvalidId` (`source.id.invalid`) lives in host-core; every
  per-source call refuses an empty, non-UTF-8 or over-127-byte ID with it
  (`SourceControlError::unmatched_id`, which the browser's own seek lookup also reports through).
  capi keeps only the pointer-level forms (null, over-long count) with the core's failure; the
  browser's over-staging count refuses with it too.

## Evidence (attempt 1)

- Gate 1: `a_refused_source_call_reports_the_same_result_and_diagnostic_on_both_hosts` in
  `hosts/host-web/tests/control_parity.rs`, 11 rows. Red with the browser writing no text and red
  with the browser's seek lookup reporting `source.id.unknown` for an empty ID.
- Gate 2: capi `failure_bytes_tests` pins `(RESULT_INTERNAL, b"source.epoch")`; host-core's
  `source_diagnostics` table gains the `InvalidId` row its exhaustive guard requires.
