# Open-issue age review — 2026-09-14

Owner request: close #130, then review remaining open issues more than one week
old and close stale issues. Review cutoff: 2026-09-07 01:15 UTC. Thirty-three
remaining open issues predate that cutoff; #559/#560 and newer issues do not.
Read-only baseline: `3a1c23e6daff74f9b76dfaf0e4911ba765f47675` on origin/main.
Evidence: live issue bodies and comments, successor states, and current source.
No benchmarks, DSP tests, product changes, or legacy-source inspection were
performed. Age alone is not a closure reason.

## Closure dispositions

The owner separately directed closure of #130. Its old wave scheduler and
worker-recovery implementation were removed; AGENTS.md and
docs/REALTIME_DEPENDENCY_POLICY.md require fresh justification for future
multicore work. Closed as not planned, preserving historical evidence.

The following seven additional issues are ready for closure after this record
is upstream. Supersession does not claim every historical acceptance gate passed.

| Issue | Disposition | Evidence and surviving ownership |
| --- | --- | --- |
| #163 | Superseded; not planned | The issue comments record delivery of banking, idle gating, interleaving and the unfused contract, then two optimization rounds. Current efficiency work is maintained in #349 and #559/#560; floor recount #368 is closed. Scheduler phase #130 is retired. No claim that every effect reached its theoretical floor. |
| #207 | Superseded; not planned | Typed headless/browser SDK and npm delivery have shipped through #320–#324, #357, #468 and later releases. SDK PCM-feed ownership #405 and recent ownership/readiness #796/#797 are closed. Current remaining DX, storage, console and analysis work stays in #379, #394, #210 and #763; the old SDK construction plan is no longer a useful execution tracker. |
| #239 | Superseded; not planned | Its #240–#246 delivery children are closed. TOML/boot-V2/SHA-256 and in-engine FLAC assumptions were superseded by canonical JSON #338, identity reset #313/#316, codec separation #356 and BLAKE3 #787. Residual lattice #291, storage #293/#394, coverage #284 and external-session evidence #338 remain open. Historical HOLDs are preserved, not relabeled PASS. |
| #268 | Superseded; not planned | Its thirteen comparison issues were closed and research synchronized at 3892a5b. The old fan-delivery train is delivered/superseded as above; its runtime program #252 was closed by the owner pending a fresh post-launch comparison. Current unfused arithmetic is recorded in docs/TARGET_MATRIX.md and guarded by the unfused seal. Retain comparison findings as history, without reopening runtime work through this tracker. |
| #354 | Superseded; not planned | The body explicitly retires its exhausted release attempt in favor of #355 then #357. #357 is closed and subsequent SDK releases are delivered. Do not execute the historical token/publication procedure. |
| #355 | Superseded; not planned | The final verdict exhausted its attempts; #354's supersession record and #357 assign the replacement Engine-only release to #357. This closure preserves FAIL evidence and does not turn the failed attempt into PASS. |
| #359 | Completed | Its final 2026-09-04 comment records the completed design/migration through PRs #361/#362, including measured CI runs. Current branch protection has exactly the `qualification` required context, matching AGENTS.md and the surviving workflow. This closes that delivered redesign, not a perpetual promise that every future CI run finishes within ten minutes. |

## Retained issues

These 26 issues remain open. Some wording is old; a live unresolved outcome,
explicit hold, or future owner decision prevents whole-issue closure.

| Issue | Reason to retain |
| --- | --- |
| #15 | De-esser remains an unimplemented product capability. |
| #17 | Dynamic EQ remains an unimplemented product capability. |
| #25 | Optional remote-control sidecar is a future capability; deleting the unused sidecars directory in #307 did not deliver it. |
| #26 | Release/listening qualification explicitly owns pending human evidence, including causal effects #737/#738/#739. |
| #27 | Third-party Wasm package/ABI conformance remains distinct from native-effect conformance. |
| #28 | Sandboxed third-party execution remains explicitly post-launch scope in AGENTS.md. |
| #124 | Source-worker implementation was delivered via #129, but its final comment explicitly retains seek-storm/throughput qualification and evidence obligations. The trace script's old binary ambiguity is fixed; that alone does not discharge the remaining work. |
| #140 | Current body and #460 evidence retain real controller/host/DSP automation integration; primitive completion does not close IO-5. |
| #147 | Unit-bearing names remain inconsistent: compressor metadata still calls the threshold parameter `threshold`. |
| #172 | Standing deterministic-Wasm-FMA watch item; no adoption or new browser/spec capability is asserted by this review. |
| #191 | Some findings are overtaken (causal compressor and analysis delivery), but other EQ/dynamics/limiter feature decisions remain. |
| #195 | Host-specific reservation/persistence follow-up is not proven completed or abandoned. Current review host is EPYC; do not apply old 7/15 cpusets or infer the owner's other machine state. |
| #197 | Owner-requested 9950X migration/rebaseline and optional W16 evaluation lack closure evidence; current review host cannot settle them. |
| #210 | Earlier strip phases delivered; N-output/PFL/live-send product obligations remain distinct. |
| #234 | Proposed perceptual-equivalence qualification remains research/future work. No accepted class-P implementation or abandonment is established. |
| #284 | Real Safari/OPFS coverage versus Linux WebKit limitations remains an explicit qualification decision. |
| #291 | Objective integer base units and client perceptual stepping remain a design, not a delivered migration; current SDK still has core/lattice.ts and lacks the proposed baseUnits/perceptual modules. |
| #293 | September 9 review explicitly identifies residual adapter storage deadlines, maintenance and verification concerns; not wholly stale. |
| #296 | Unscheduled client token-layer proposal remains possible future work, dependent on #291. |
| #338 | Canonical JSON implementation delivered, but the issue was explicitly reopened for real external-session boot evidence or a numbered successor. Do not erase that hold during housekeeping. |
| #349 | Active efficiency audit with maintained successor ownership #559/#560 and recent delivered findings; not an abandoned historical queue. |
| #377 | #758/#762 fixed probe attribution, but structured decoding/platform decisions remain explicitly open in the September 12 update. |
| #379 | Some DX fixes shipped (#393/#428 and later SDK work); the larger proposal still contains unresolved API/agent-integration decisions. |
| #382 | Handwritten JSON emission remains in tools/parameter-metadata/src/abi_layout.rs; deferred generator work remains applicable. |
| #391 | Engine sentinel is delivered, but September 12 evidence explicitly leaves human upstream submission outstanding. |
| #394 | hosts/host-web/web/stem-store still exists; adapter/app consolidation has not discharged the engine-copy retirement condition. |

## Delivery

Publish this documentation checkpoint, synchronize the closure notes with the
matching existing issue specs, then close and reread the seven remote states.
No issue is reported complete merely because this review selected it for closure.
