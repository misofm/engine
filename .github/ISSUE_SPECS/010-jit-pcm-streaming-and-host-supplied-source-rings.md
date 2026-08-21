# 010 JIT PCM streaming and host-supplied source rings

## Outcome

Provide bounded just-in-time sources so stems are never fully resident solely for playback.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Required engine rates are 44,100, 48,000, 88,200, 96,000, 176,400, 192,000, 352,800, and 384,000 Hz; source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement native WAV/RF64 metadata/read/decode workers, per-source bounded SPSC PCM rings, generation-tagged seeks, prefetch policy, zero-fill/underrun counters, and equivalent host-supplied chunk contract for browser/mobile.

## Required public interfaces/contracts

`PcmSourceRing` exposes frame capacity, generation, write/read cursors and underrun count; `SourceCommand::Seek { generation, frame }`; `HostChunkProvider` submits explicit-rate planar chunks only outside render.

## Deliverables

WAV/RF64 parser/worker, rings, seek protocol, memory-limit configuration, host chunk adapter, long-stem fixtures and telemetry.

## Explicit non-goals

Full-stem preload, compressed-format catalog, render-thread filesystem/network/decode, implicit sample-rate conversion, or lossless recovery after injected underrun.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Hazards/decisions

Underflow is deterministic zero plus counter. Browser baseline has no Rust filesystem: https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html.

## Acceptance gates with objective measurements

A sparse multi-hour source and a one-minute source with identical ring settings allocate exactly the same PCM ring bytes and differ only by measured bounded parser/OS metadata recorded before implementation; injected I/O delay never blocks render, outputs exact zeros for missing frames and increments the counter/event; stale-generation seek chunks are discarded under randomized races; every advertised WAV/RF64 PCM format/layout fixture converts bit-exactly where representable and otherwise to correctly rounded `f32` within 0.5 ULP; source/engine-rate mismatch returns a typed error.

## Target matrix

Native WAV/RF64 workers; iOS/Android/browser provide host chunks to the same ring contract.

## Required evidence

RSS/capacity measurements, seek-race tests, decoded PCM checksums, underrun trace, and render allocation audit.
