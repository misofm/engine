# 073 Native PCM reference runner and C ABI qualification

## Outcome

Complete and qualify the stable ABI V1 without changing it: add the native WAV/RF64 reference PCM
runner, complete Issue-005 provider/mutation behavior behind the frozen command entrypoint, and seal
the supported native ABI/runner matrix.

## Context

Issue 022 owns the launch-sized immutable host-fed C ABI product. This stateless successor owns only
the separable native file-runner, complete control-provider integration, golden output and platform
qualification needed to reach the former combined issue's end state. It may not redesign the ABI,
DSP, graph, source, session or protocol contracts.

## Scope

- Implement a headless reference runner/library that loads strict TOML, resolves declared native
  WAV/RF64 sources off render, renders through ABI V1, and writes deterministic raw planar `f32`.
- Complete the V1 BTLV provider behind `miso_engine_v2_submit_command`, including transactional
  session edits through off-render replacement-plan compilation/publication and bounded response,
  event, replay and backpressure behavior.
- Qualify installed header/library discovery, symbol/version/layout compatibility, runner failures,
  native source lifetime, plan swap/retirement, Linux plus supported macOS/Windows builds, and the
  mobile-native compile boundary consumed by Issue 023.

## Explicit non-goals

ABI V2, new symbols or struct fields, new DSP/source/graph semantics, browser AudioWorklet, mobile
example applications, network service, codecs, a benchmark, timing, or listening.

## Dependencies by exact issue title

- Stable C ABI and host-fed planar PCM render

## Acceptance gates

- Golden strict sessions at exactly 44.1/48/88.2/96 kHz resolve WAV and RF64, render deterministic
  PCM identical to the accepted direct V2 path, preserve fixed latency/tail/bypass/PDC and report
  source mismatch without SRC.
- Every Issue-005 V1 command/response/event family passes through caller-owned ABI buffers; structural
  edits publish only complete replacement plans, full retirement defers rather than frees on render,
  and saturation/replay/revision behavior matches the protocol corpus.
- Missing/malformed files, truncated RF64, invalid session, short output, interrupted write and
  output overwrite refusal leave deterministic diagnostics and no accepted partial artifact.
- Installed header/static/shared libraries and exact symbols/layout/version pass C11/C++17 consumer
  smoke on Linux and build on supported macOS/Windows/mobile-native targets; no Rust ABI leaks.
- Representative runner and one-million ABI render/swap calls pass non-timed realtime and ownership
  audits. No benchmark or threshold is permitted.

## Attempt and evidence policy

Permit one Terra attempt and one bounded Sol correction; a second failure stops. Record runner PCM
hashes, protocol corpus results, source/swap/drop ownership, ABI/platform logs, exact artifact hashes,
zero realtime violations and strict PASS/FAIL. Timed/benchmark invocation count is zero.

