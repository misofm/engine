# Prevent managed live spectrum starvation under asynchronous browser polling

Discovered while executing misofm/app#210 from the misofm/engine#796 handoff.
Smallest outcome: the existing managed spectrum stream publishes valid settled
windows during ordinary uninterrupted browser rendering, without false gaps
caused by SDK polling cadence. Preserve true gap/loss accounting and all current
ownership, resource, target-switch and stale-result contracts.

## Reproducer and bounded scope

Published SDK 0.2.5, adapter 0.5.7; actual packaged Worker/AudioWorklet in the
existing app launch-browser fixture, 48 kHz, quantum128, N/hop2048, 750 Hz,
input L=.1/R=.05, +6 dB EQ, otherwise neutral. Over eight seconds there were
73 gap notifications, no ready window (sequence/windows0), droppedCaptures218,
sourceUnderrun=false throughout. Evidence is preserved at
/tmp/miso-796-audit/app210-browser-status.log and app210-browser-status/results.json.
The same analyzer passes fixed numeric tolerances when the existing node realm
is paced within capture capacity. SDK observation-subscriptions.ts currently
sets nextCaptureAt after awaiting the read; its periodic poll can skip a capture
because of async completion latency. Confirm the exact cause before fixing.

Limit implementation to the existing SDK observation polling/read scheduling
and directly necessary focused tests. No DSP/Wasm/ABI changes, app timing
workaround, increased capture buffers, new framework, benchmark matrix or
relaxed availability/loss/numerical acceptance. If the demonstrated cause lies
outside this boundary, report and amend this brief before implementation.

## Gates and workflow

Luna XHIGH implements one minimal coherent attempt; separate fresh Astra MEDIUM
adversarially verifies and fixes concrete bugs only. Root coordinates and
checkpoints focused-green exact paths before more work. Max five coherent
attempts with one verdict each. Integrate latest main before landing.

Add a deterministic regression where asynchronous read completion must not
perpetually skip successive native capture windows; retain bounded one-in-flight
polls, truthful gap/drop counters, close/replacement and target switching.
Run focused SDK tests/types plus proportional package gates. Reuse the failing
actual packaged browser scenario to prove valid continuous windows, unchanged
numeric tolerances and focus/fader behavior; preserve failure and success logs.
No accepted DSP requalification; accepted Wasm digest must stay unchanged.

## Release/dependency map

Source correction PASS and required main CI precede a new immutable SDK patch
through existing engine#794 qualify/publish/verify workflow (reopen/amend its
release record; never republish0.2.5). Re-audit unused version (candidate0.2.6).
Adapter#95 then adopts exact corrected SDK and cuts an unused patch (candidate
0.5.8) through its existing release workflow, preserving one SDK resolution.
App#210 adopts verified registry identities, reruns its existing browser gate,
obtains final PASS, merges/deploys and verifies live build/provenance. Existing
0.2.5/0.5.7 provenance and acceptance records remain valid for those bytes;
new bytes require new evidence. Releases/deployment are already user-authorized.
Close this bug issue only after source PASS, merged evidence and published SDK
verification; synchronize all amended GitHub bodies/states before reporting done.

## Attempt 2 source/candidate PASS

Fresh Astra MEDIUM accepts checkpoint `a0949481`, including `087e9d11` cadence
correction. Attempt 1's exact served candidate still starved: native gap reads
report new drops before popping the full queue. Attempt 2 performs at most one
immediate sequential recovery read after an automatic gap, preserving delivered
and coalesced loss, older queued metadata, epoch resets and one in-flight poll.
Initial stale-Vite-cache invocations were invalid candidate evidence and are
preserved separately; rebuilt attempt-1 failure and uninstrumented attempt-2
success have verified served-byte attribution.

Full SDK 250/250 with zero skips, types and package qualification PASS. Existing
actual Chromium Worker/AudioWorklet scenario PASS: spectrum L/R
-14.0000009537/-20.0206012726 dBFS, response +6/+6 dB, meters
0.1995262504/0.0997631252 with overlapping actual spans. A -6 dB fader produces
-20 dBFS/.10000000149 amplitude without changing response. Focus switches to
track-000 while context stays running and transport advances; errors empty.
Report `/tmp/miso-801-verifier/attempt2-verdict.md` and
`browser-attempt2/results.json` preserve evidence. Candidate archive SHA256
`02fb8c8aa477594f9b918d1bd5fbb7e1a4597a94a3ba7b770313c34bbac7f80a`;
served subscription module SHA256
`21cad800ce02b47d2eeef21323524d54a25a35b37e567790886e95d33d7a01cc`.
Accepted Wasm remains
`c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
The local candidate retains development version 0.2.5 and MUST NOT be published;
new immutable metadata, required main CI and release verification remain pending.
