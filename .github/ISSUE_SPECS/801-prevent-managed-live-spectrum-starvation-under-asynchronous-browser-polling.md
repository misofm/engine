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

## Required browser fixture correction

PR #802 CI run 34803508846 found a snapshot race in the existing continuous
spectrum fixture. Exact local reproduction shows all numeric/PCM/meter/span
predicates pass, but callbackCount=2 while the copied statuses list contains
only gap: it is copied before the new bounded recovery publishes ready.
Evidence `/tmp/miso-801-verifier/ci-chromium-raw.json`. Scope is amended only to
`hosts/host-web/qualification/sdk-response-entry.ts`: retain the live callback
notification list, already populated by automatic and manual pump publications,
and remove redundant copied/manual-deduplicated collection. Keep all existing
run.mjs predicates and tolerances unchanged. No SDK, DSP or package bytes change;
source attempt-2 numerical acceptance stands, required CI remains pending.

## Corrected SDK 0.2.6 published and verified

PR #802 merged source `cdf629d6bfd0224b3532dd0abd04b9581240da56`; required
PR qualification 34803929416 and exact-main qualification 34804388382 PASS.
Qualification 34804755779 built and preserved one immutable 0.2.6 archive.
Original OIDC publish 34804976202 accepted/signed the package; its 60-second
registry-convergence check timed out while npm processed it. No republish occurred.
After registry convergence, verify-only 34805198433 PASS using the same qualified
archive. Registry public version/latest are 0.2.6 and downloaded bytes match exactly.

Archive SHA256: `8219178d591c76d820d7ad2e2f7b894fe7f39185f89667f59c675fad603b81eb`.
SHA1: `35569fbe5fb61626abca04b64d93b852b5b38748`.
Integrity: `sha512-IL3x8280+G75SlLxeznUwFwIve+uoHgf5OVXNid+PTsZneJX/k94rXGWhBTYwutCaWiLQf6N0cr9ZEH7AaTZ3g==`.
Accepted Wasm SHA256 remains
`c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
Fresh Astra MEDIUM independently verified fresh public imports, strict types,
CLI 0.2.6, npm 11.19.0 signatures and verified SLSA DSSE binding the exact PURL,
archive SHA512, trusted engine workflow/main source and original invocation
`34804976202/attempts/1`. Report `/tmp/miso-801-verifier/registry026-verdict.md`;
machine acceptance/provenance/signatures and registry archive are preserved
beside it. Source/candidate/package/registry acceptance is PASS.
Adapter #95 now adopts exact SDK 0.2.6 in its separate immutable patch before
app #210 final adoption and deployment. Prior 0.2.5 evidence is retained.
