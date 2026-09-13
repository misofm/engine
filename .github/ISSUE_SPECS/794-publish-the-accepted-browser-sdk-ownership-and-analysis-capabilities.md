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
