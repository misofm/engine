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
