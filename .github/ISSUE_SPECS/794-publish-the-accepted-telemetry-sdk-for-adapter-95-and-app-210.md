# Publish the accepted telemetry SDK for adapter #95 and app #210

## Requirements

Publish one new immutable `@misofm/engine` version containing accepted #789,
#791 and #793 capabilities. This is release metadata and execution only:
no telemetry features, DSP/transport refactor, release framework, performance
campaign or new browser matrix. Publication is already authorized as necessary
for adapter #95 and app #210 delivery.

1. Release-metadata implementation waits for independent #793 PASS and creation
   of this issue. It may then continue on the accepted feature branch/worktree,
   after its editor hands off, as an exact-path metadata checkpoint. Deliver
   accepted #793 plus release identity in one coherent PR/main merge; a separate
   release worktree or metadata-only main merge is not required. Preserve other
   work, refresh/integrate current main before landing, and run required CI.
2. Choose an unused version after checking the registry immediately before
   freezing metadata. Read-only registry inspection on 2026-09-13 reports
   `version` and `latest` both 0.2.4. Do not assume the next version remains
   unused or overwrite an existing version.
3. Package code, declarations, generated ABI/catalog, Worklet/Worker assets and
   Wasm must come from the same accepted source closure. Preserve canonical
   BLAKE3 PCM semantics and all accepted functionality. Record merged source
   SHA, Wasm SHA-256, archive SHA-256/SHA-512 integrity, version and workflow
   run IDs. No hand-copied artifacts or source-only release claims.
4. Use the existing qualify → publish → verify workflow on one exact merged
   main SHA. Publish the exact qualification tarball without rebuilding or
   repacking it. Keep all existing security, ancestry, immutable-version and
   cryptographic provenance checks intact.

## Minimum metadata change

Read-only candidate inspection at `67ec843182e4abddf284af7a6ff72ffeff7a5dfa`
is not acceptance; reviewer work remains active. Update only:

- `sdk/package.json` version and both package-version locations in
  `sdk/package-lock.json` (root and `packages[""]`), preserving dependency pins.
- `.github/workflows/npm-publish.yml`: `PACKAGE_VERSION`, release job display
  name, qualification-job-name filter, packed-version guard, and exact DSSE
  package PURL guard. Synchronize every live 0.2.4 literal in this workflow.
- Its `EXPECTED_WORKLET_SHA256` must match accepted
  `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` exactly.
  The workflow still pins old `47d12d99…`; the current candidate file contains
  `c1191d67052806984441eca262d6678583f36f88eaec9f7495a3580d4d81c7b4`.
  Re-read after PASS; never change the source-of-truth pin just to satisfy
  publication. If Wasm is unchanged, reuse its accepted evidence.
- The synchronized numbered local/GitHub release issue and concise decision
  record. Root creates these before implementation; no issue is created by
  this brief.

Record the independently accepted feature checkpoint and the following
metadata-only checkpoint separately. Continue normal checkpoint pushes; this
combined delivery does not introduce CI-conscious batching or defer authorized
pushes. Open/update the coherent PR when accepted code and release identity are
ready. Reuse feature acceptance only while its relevant code/assets are unchanged;
main integration or review fixes require the affected gates. Required PR/main
CI remains binding, and both issues retain their own synchronized evidence.

Keep `scripts/sdk-package.sh` and `sdk/codegen/stage-package.mjs` unchanged
unless a concrete release blocker requires rebriefing. Existing staging derives
the exact asset closure and manifest digests; generated provenance is not
hand-authored. Keep dist, tarballs and caches outside commits.

## Qualification and publication

1. Audit the small metadata diff and workflow syntax. Reuse accepted feature
   gates when source/assets are unchanged. For a local package check use
   `scripts/sdk-package.sh check <accepted-artifact-directory>` to avoid an
   unnecessary Wasm rebuild; it runs the existing unpacked-tarball smoke.
   `sdk/test/package-tarball-smoke.mjs` already creates a fresh consumer,
   checks strict public declarations/imports, assets and real headless Wasm.
   Confirm accepted collection/managed APIs are present in the packaged public
   surface. Actual browser coverage is opt-in there; do not describe its default
   run as browser evidence. Reuse accepted matching-artifact browser evidence.
2. After the combined feature/release PR lands and required CI passes, dispatch
   `.github/workflows/npm-publish.yml` with `mode=qualify`, exact
   `expected_sha`, and no qualification run ID. Its existing pinned toolchain
   builds/verifies the Wasm once, runs generated/deletion/type/headless/package
   gates, packs one archive, smoke-tests/dry-runs it and uploads immutable
   evidence. Do not bypass this workflow build with local evidence.
3. Dispatch `mode=publish` at that same SHA with the successful qualification
   run ID. Preserve PR #353 accepted ancestry and rejected-commit checks,
   trusted workflow/run/artifact identity, OIDC-only publishing, token-fallback
   refusal and exact tarball checksums. If publication is ambiguous, recover
   with `mode=verify` and the same qualification run, never another publish.
4. Completion requires matching registry version, public access, latest tag,
   shasum/integrity, fresh registry imports/CLI and verified SLSA attestation
   binding package bytes, trusted workflow and exact source SHA. These checks
   already exist. Synchronize issue evidence/closure and give adapter #95 the
   exact published version/source/archive identities; app #210 waits for its
   matching adapter release too.

Implementation checkpoint: Luna max verified registry versions/latest still end at0.2.4 and selected unused0.2.5 (/tmp/issue794-registry-versions.log, /tmp/issue794-registry-tags.log). Only package version, both lock versions, and existing workflow version/accepted-Wasm guards changed; dependencies, product code and release security checks are unchanged. JSON/YAML checks and package/generated/build/enginectl/tarball smoke pass against accepted /tmp/issue793-candidate1-artifact (/tmp/issue794-package-check.log). Publication remains pending required CI, one combined main merge, and existing qualify/publish/verify workflow.

Independent Astra medium review, attempt 1: PASS for source/package release readiness at `53520b3d76bdbe35514679cc1a7a282408c4680a`. Normalization confirms only the authorized version and Wasm-pin substitutions; dependency pins, exact-tarball/OIDC/attestation/ancestry safeguards are unchanged. Package smoke passed. No correction was needed. Full review is preserved at `/tmp/issue794-astra-medium-review-attempt1.md`. This is not publication acceptance: required CI, merged-main qualify/publish/verify and registry provenance evidence remain pending.
