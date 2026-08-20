# Engine V2 implementation plan

## Purpose

This is the concise index for the stateless implementation payloads in [`.github/ISSUE_SPECS`](../.github/ISSUE_SPECS/README.md).  The issue bodies, not this roadmap, are the source of truth.  Engine V2 is greenfield: do not inspect or copy V1/legacy designs.

## Delivery sequence

1. **Foundation and proof:** 001–005 establish the workspace, research corpus, realtime memory model, TOML compiler, and control protocol.
2. **Render engine:** 006–010 compile the graph, builtins, SIMD banks, deterministic native waves, and bounded source streaming.
3. **Effects:** 011 defines only the launch native runtime contract; 012–021 implement the launch processors using it. Graph issue 006 and launch effects do not wait for external package or persisted-state formats.
4. **Deployment:** 022–025 provide the C runner, mobile/native examples, browser AudioWorklet, and optional remote sidecar.
5. **Qualification and extensibility:** 026 qualifies a release; 029 freezes canonical descriptor/package/CID/state interchange; 027 layers the third-party Wasm ABI on that identity contract; 028 is explicitly post-launch execution. Dependency order, not numeric order, controls 029 -> 027 -> 028.

## Non-negotiable release shape

All rendering is planar `f32`.  The render thread exclusively owns a preallocated prepared plan whose topology/capacities are immutable and whose DSP state is mutably processed without allocation; displaced plans are reclaimed off-thread.  No compiled track ceiling exists.  Tracks are dual-mono and run `input -> polarity/trim/HPF/LPF -> SIMD1 -> dynamic -> SIMD2 -> fader/mute -> matrix/pan -> routes`.  Native effects are in launch scope.  Third-party Wasm is **not** launch scope and belongs only to the dynamic rack.

The required rate set is 44.1/48/88.2/96/176.4/192/352.8/384 kHz.  Sessions are strict versioned TOML; output is PCM.  Any source, host, or control work that can allocate, block, decode, parse, log, or use I/O is outside the render path.

## Review cadence

For every issue: Sol approves brief/gates; Terra produces attempt 1 with evidence; Sol adversarially reviews; Sol may revise up to two times; after three total failed attempts, rescope/rebrief rather than relaxing gates.  A gate that cannot be evaluated before coding fails briefing.  Weekly optimization work must become an issue with a baseline and a measured result.

## Index

The checked-in control-plane documentation for issue 005 is indexed in [the documentation README](README.md), including the BTLV wire contract, registry, sizing guide, provider boundary, and non-benchmark conformance record.

| Range | Outcome |
| --- | --- |
| 001–005 | buildable contracts and safe control-plane foundation |
| 006–010 | runnable no-track-limit mixer core and bounded streaming |
| 011–021 | bounded native runtime contract and documented launch effect set |
| 022–025 | native, mobile, browser, and optional cloud control deployment |
| 026–028 | release proof, third-party ABI kit, then post-launch sandbox |
| 029 | canonical descriptor/package/CID/state interchange prerequisite for 027 and future repository work |
