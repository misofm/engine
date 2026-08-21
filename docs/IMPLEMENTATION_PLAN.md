# Engine V2 implementation plan

## Purpose

This is the concise index for the stateless implementation payloads in [`.github/ISSUE_SPECS`](../.github/ISSUE_SPECS/README.md).  The issue bodies, not this roadmap, are the source of truth.  Engine V2 is greenfield: do not inspect or copy V1/legacy designs.

## Delivery sequence

1. **Foundation and proof:** 001–005 establish the workspace, research corpus, realtime memory model, TOML compiler, and control protocol.
2. **Render engine:** 006–010 compile the graph, builtins, SIMD banks, deterministic native waves, and bounded source streaming. Issue 006 is accepted under its recorded post-stop Sol rescope. Issue 007 stopped after three attempts and retains only its proven scalar DSP/runtime slice; issue 032 centralizes the four-rate launch policy. Issue 034 then stopped after landing bounded builtin contract corrections; issue 036 owns their numerical boundary and final acceptance before issue 008. Issue 008 does not wait for issue 035's scalar qualification tooling. Issue 010 also follows issue 032; issue 009 follows issue 008.
3. **Effects:** 011 defines only the launch native runtime contract; 012–021 implement the launch processors using it. Graph issue 006 and launch effects do not wait for external package or persisted-state formats.
4. **Deployment:** 022–025 provide the C runner, mobile/native examples, browser AudioWorklet, and optional remote sidecar. Issues 022–024 wait for issue 035's corrected builtin machine candidate.
5. **Qualification and extensibility:** 026 qualifies a release only after issue 035 machine qualification and issue 033 real human listening; 029 freezes canonical descriptor/package/CID/state interchange; 027 layers the third-party Wasm ABI on that identity contract; 028 is explicitly post-launch execution. Dependency order, not numeric order, controls 029 -> 027 -> 028.
6. **Nonblocking tooling follow-up:** 030 hardens the graph benchmark shell wrapper and promotes the exact preserved validator-valid issue-006 raw artifact. It does not gate 007–010, does not change graph acceptance, and requires no further issue-006 benchmark run.
7. **Rate-policy correction:** 032 makes 44.1/48/88.2/96 kHz the exact launch session/render tier and preserves 176.4/192/352.8/384 kHz only as an informational compatibility corpus. Dependency order, not numeric order, controls 007 accepted slice -> 032 -> 034 corrections -> 036 -> 008, while 010 follows 032; downstream effects, hosts and release qualification follow their exact dependencies.
8. **Builtin post-stop correction:** 034 stopped after two attempts with bounded metadata, sealed-only graph integration, exact checked resource accounting and compiler-matrix corrections landed. Issue 036 owns the representable TPT cutoff domain and final nonbenchmark acceptance. Then 008 and 035 may proceed independently. Issue 035 alone owns complete builtin fixtures, direct/graph audits, target evidence and one eventual externally invoked benchmark containing exactly two internal rounds. Its invocation count starts at zero and it gates 022–024, 033 and 026.

## Non-negotiable release shape

All rendering is planar `f32`.  The render thread exclusively owns a preallocated prepared plan whose topology/capacities are immutable and whose DSP state is mutably processed without allocation; displaced plans are reclaimed off-thread.  No compiled track ceiling exists.  Tracks are dual-mono and run `input -> polarity/trim/HPF/LPF -> SIMD1 -> dynamic -> SIMD2 -> fader/mute -> matrix/pan -> routes`.  Native effects are in launch scope.  Third-party Wasm is **not** launch scope and belongs only to the dynamic rack.

The launch-supported session/render rate set is exactly 44.1/48/88.2/96 kHz. The 176.4/192/352.8/384 kHz corpus is retained only for extended compatibility research and is not launch support. Sessions are strict versioned TOML; output is PCM. Any source, host, or control work that can allocate, block, decode, parse, log, or use I/O is outside the render path.

## Review cadence

For every issue: Sol approves brief/gates; Terra produces attempt 1 with evidence; Sol adversarially reviews; Sol may revise up to two times unless the issue brief freezes a smaller budget; after that budget fails, rescope/rebrief rather than relaxing gates. Issue 034 is stopped; issues 035 and 036 each permit only two total attempts. A gate that cannot be evaluated before coding fails briefing. Weekly optimization work must become an issue with a baseline and a measured result.

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
| 030 | nonblocking graph benchmark runner hardening and accepted-artifact promotion |
| 031 | optional post-foundation higher-precision builtin-filter investigation |
| 032 | four-rate launch authority and extended-rate compatibility deferral; ordered after 007 and before 008/010 |
| 033 | real issue-007 builtin filter/matrix human listening after issue 035 seals the machine candidate and benchmark |
| 034 | stopped after bounded launch-critical contract corrections; no PASS and no timed benchmark |
| 035 | complete builtin qualification fixtures/audits/targets and sole one-invocation/two-round benchmark owner; follows 036 and gates 022–024, 033 and 026 |
| 036 | representable TPT cutoff domain and final acceptance of issue-034 contract corrections; prerequisite for 008 and 035; no timed benchmark |
