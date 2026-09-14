# Publish the accepted browser SDK ownership and analysis capabilities

Amends the existing release issue; no duplicate release issue. Product inputs:
accepted misofm/engine#789/misofm/engine#791/misofm/engine#793 and misofm/engine#796. Downstream:
misofm/engine-web-adapter#95 and misofm/app#210.

## Current state and smallest outcome

Publish one immutable SDK package containing the accepted focused analysis and
new canonical browser measurement owner. On 2026-09-13 engine main
`69c268f240bf30b2a43b43dd521120cd89dcc0b9` contains accepted misofm/engine#793/PR #795 and
source metadata for 0.2.5; registry latest remains 0.2.4 and 0.2.5 is unpublished.
This issue remains OPEN. Source/package readiness PASS of the prior metadata
does not qualify newly changed SDK bytes or prove registry publication.

## Release cut and permitted preparation

1. Hold publication until misofm/engine#796 independently passes and its source
   is merged with required CI. Recheck 0.2.5 immediately before freezing the
   package; reuse it if unused. If another publisher has used it, choose an
   unused version and update every existing exact-version guard. Never overwrite.
2. No dependency on engine misofm/engine#797 or adapter backend/PCM successors.
   Already accepted compatible sibling source may enter the cut only if it is
   ready before qualification starts; do not delay plotting for it. Freeze and
   record the included issue/source set. Do not reopen accepted DSP/analysis
   scope or force more numerical/browser matrices for TypeScript ownership work.
3. Adapter/app preparation may use an exact accepted local packed candidate
   with its source/archive identity recorded and temporary installs uncommitted.
   This is preparation, not registry acceptance. Final adapter publication/app
   deployment require verified published dependencies; no overrides or copied
   artifacts replace that proof.

## Existing release mechanism

Use `.github/workflows/npm-publish.yml` qualify → publish → verify at one exact
merged main SHA. Qualification builds/verifies its own closure with the pinned
toolchain, runs existing generated/type/headless/package checks, packs once and
preserves immutable evidence. Publish that exact tarball without rebuilding or
repacking; after ambiguous publication, verify with the same qualification run.
Keep PR #353 ancestry/rejected-commit checks, workflow/run/artifact identity,
checksums, OIDC-only policy, token refusal and SLSA/DSSE checks unchanged.

`sdk/package.json`, both root/package version locations in `sdk/package-lock.json`
and existing workflow literals are already 0.2.5; do not churn them if unchanged.
If version changes, synchronize package env/display/filter/packed-version/PURL
guards and their existing fixtures. `EXPECTED_WORKLET_SHA256` must match the
accepted `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`.
TS-only ownership work should leave Wasm unchanged; verify the actual digest,
do not assume it. Reuse accepted local artifacts for package checking; the
existing release qualification build is still required. Do not add build loops.

## Gates and complete evidence

- Audit only necessary metadata changes and affected workflow fixtures. Existing
  `scripts/sdk-package.sh check <accepted-artifact-directory>` and
  `sdk/test/package-tarball-smoke.mjs` must include strict imports/types for the
  new measurement API alongside accepted collection/managed analysis APIs.
  Default tarball smoke is not browser evidence; use the ownership issue's
  matching-artifact browser proof and preserve accepted Worker/Worklet tests.
- After required merged-main CI, qualify once, then publish the exact successful
  archive at that SHA. Record source SHA, included capability issues, SDK version,
  Wasm SHA-256, archive SHA-256/SHA-512 integrity and workflow run IDs.
- Completion requires registry public access, exact version/latest tag and
  shasum/integrity, fresh registry public imports/CLI, and verified attestation
  binding exact package bytes, trusted workflow and source SHA. Give misofm/engine-web-adapter#95/misofm/app#210
  these identities. Source version strings or a local tarball are insufficient.

Luna XHIGH implements any necessary release metadata. A fresh Astra MEDIUM
independently verifies concrete correctness/security regressions and fixes only
in-scope bugs; a separate fresh Astra MEDIUM coordinates. Maximum five coherent
attempts, one adversarial verdict each, then stop/rebrief. Root checkpoints,
integrates latest main, runs required CI, records exact evidence and synchronizes
local/GitHub issue state. Close only after registry/provenance PASS and verify
remote CLOSED. Publication was already user-authorized for this delivery.

## Preserved decision record

Prior release-metadata implementation selected 0.2.5 after registry inspection;
independent source/package review passed at
`53520b3d76bdbe35514679cc1a7a282408c4680a`. PR #795 merged accepted analysis,
metadata and bounded CI corrections into the main baseline above. Those facts
remain evidence for their exact source; fresh misofm/engine#796 package bytes
must qualify again. Root retains existing linked CI/review evidence when applying
this amendment. This brief does not claim new execution/publication results.


## Fresh release preparation and source verification

Luna XHIGH audited current 0.2.5 metadata and found no version/pin changes
necessary: registry 0.2.5 is unused, all exact guards agree, and accepted Wasm
SHA-256 is `c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
Fresh Astra MEDIUM independently records source/package-readiness PASS after
a narrow packed strict-consumer correction adding accepted collection and
managed-observation/response/spectrum type references alongside new measurement
types. Updated packed smoke and existing publish-mode fixtures pass; no runtime
change or Wasm rebuild. Review `/tmp/miso-796-audit/verify-794.md`, packed proof
`verify-794-packed.log`. The local preparation archive SHA-256
`f920ab50c69a04e24c9ad7474c80c021251e3d49b87fab43774549fb2f8f12e4` is not the
registry release archive. Final merged source/required CI, exact workflow qualify
then publish, and independent registry/archive/provenance verdict remain pending.


## Combined source cut

The independently accepted #797 helper was ready before release qualification
and merged without conflicts into the accepted #796 candidate at
`f78d599e0a51b49daa0d5def7c01e9b8a8aaeb41`. Refreshed main remains
`69c268f240bf30b2a43b43dd521120cd89dcc0b9` and is already an ancestor. Root's
combined SDK gate passes 248/248; types, generated/package and real packed
Vite/Chromium meter/boot/seek checks pass using the unchanged accepted artifact.
Logs: `/tmp/miso-796-audit/combined-types.log`, `combined-sdk.log`,
`combined-package.log`. The 0.2.5 release input set is accepted #789/#791/#793
plus #796 and #797; no further sibling work is needed for this cut. Required
PR/merged-main CI and #794 immutable registry publication remain pending.


## Published delivery — CLOSED

Fresh independent Astra MEDIUM final release verdict: **PASS**. Public
`@misofm/engine@0.2.5` and `latest` identify the exact qualified archive.
Source is merged main `1646a6a1bd0011cc2b5480283bf498b27be460e5` (PR #798);
required main CI 34792194652 PASS. Included capabilities are #789/#791/#793,
#796 measurement ownership and #797 PCM readiness. Qualification run
34792550685 PASS. OIDC publication 34792751578 signed/accepted the archive but
registry processing exceeded its verification window; **no second publication**
was attempted. Verify-only recovery 34792987814 PASS after propagation.

Archive SHA-256: `c26208470b5409ad789085d251d94bdf1117b32dcd3b5e544f8696dffffe3d4d`.
Registry SHA-1: `234bdf5885aeeaf04d20ed8ab3306cd4fa3edf21`.
Integrity: `sha512-cNMuslg9t7NAkBrYFMl8jcd1fOy5/fN+FEeDE+WdiMj+8+DkWTkvbWvMo1UMWgC+MrnX4BKhYz2QrKmAstIWwQ==`.
Wasm SHA-256 remains `c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
Fresh registry imports/CLI, public access and cryptographically verified SLSA
provenance bind exact package bytes, trusted npm-publish workflow/main ref and
the source SHA. Independent record `/tmp/miso-796-audit/verify-794.md`; exact
qualified archive/evidence `/tmp/miso-796-audit/release025-qualification`;
independent registry/attestation evidence `/tmp/miso-796-audit/verify794-registry`.
Downstream #95/#101 and app #210 may now use this verified registry identity.

## Confirmed downstream live-capture correction release

Reopened for the narrowly scoped engine#801 defect discovered by the required
app#210 real-browser gate. The published0.2.5 provenance above remains valid,
but live managed spectrum yielded73gap notifications/218drops/windows0 without
source underruns. A new immutable SDK patch is required after #801 source PASS
and required merged-main CI. Re-audit unused version (candidate0.2.6), update
existing exact guards only, and reuse the same qualify/publish/verify workflow.
No DSP/Wasm changes or expanded acceptance matrix. Preserve prior archive/run
identities and record new source/archive/registry/provenance identities separately.
Then adapter#95 cuts its exact-dependency patch before app#210 final adoption and
deployment. This is the existing authorized release scope's defect correction;
publication is not yet complete for the corrected bytes.

Final correction metadata checkpoint `e45323a6` prepares unused SDK 0.2.6.
Fresh Astra MEDIUM independently verified exactly four metadata/fixture paths,
publish-mode security gates, and a packed 0.2.6 archive. All 96 non-package.json
files are byte-identical to the browser-accepted `a0949481` candidate; the sole
package.json change is version 0.2.5 to 0.2.6. Candidate SHA256:
`8219178d591c76d820d7ad2e2f7b894fe7f39185f89667f59c675fad603b81eb`.
No source/codegen/Wasm changes. Existing exact-main/SHA, qualified archive,
immutable-version refusal, OIDC-only publish, SLSA PURL/SHA512/workflow/gitCommit
checks remain intact. Evidence under `/tmp/miso-801-verifier/release026-*`.
Required main CI and actual qualify/publish/verify are still pending.

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
