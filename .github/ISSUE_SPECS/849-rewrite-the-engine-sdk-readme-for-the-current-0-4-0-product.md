# Rewrite the Engine SDK README for the current 0.4.0 product

## Smallest closable product slice

Replace the published `sdk/README.md` with a current, task-oriented guide for `@misofm/engine@0.4.0`. The README should explain what ships, the supported execution models, the package entry points, installation, one minimal headless example, one minimal browser example, semantic control and acknowledgement timing, meter/spectrum/response observation, artifact integrity, and the boundary between engine PCM output and caller-owned delivery.

## Exact path boundary

Modify only:

- `sdk/README.md`
- `.github/ISSUE_SPECS/849-rewrite-the-engine-sdk-readme-for-the-current-0-4-0-product.md`

Do not change code, generated files, manifests, package metadata, architecture documents, or release artifacts.

## Content requirements

Derive every claim from the current source, declarations, package manifest, Engine V1 architecture, and accepted issue records. Lead with the current product instead of release archaeology. Keep examples short, compilable in shape, and limited to public package entry points. Clearly state realtime ownership, source-delivery ownership, exact package asset identity, supported sample rates, command acknowledgement semantics, and observation timestamp semantics. Describe protected observation as an additive bounded host capability without implying DRM, encrypted PCM, or app activation. Link detailed policies and designs rather than duplicating them.

Remove stale or overly long historical explanations that obstruct onboarding. Preserve material limitations and compatibility boundaries. Do not claim unsupported platforms, codecs, third-party execution, implicit sample-rate conversion, or completed capabilities still tracked by open issues.

## Objective gates

- Verify every import and public symbol in examples against the current TypeScript declarations.
- Run `npm run check:generated`, `npm run check:assets`, and the package type/public-surface gate appropriate to documentation-only changes.
- Run `git diff --check`.
- Obtain a fresh adversarial review before push.

## Evidence

Baseline was synchronized Engine main `704b8a3dddd3c91c6cbcb98c869ba67c4c0ae39e`.

- A fresh Astra XHIGH author replaced the 626-line release-history guide with a 263-line,
  task-oriented guide using only the four public package entry points.
- `npm run check:generated`, `npm run check:assets`, `bash scripts/check-sdk-types.sh`, and
  `git diff --check` passed.
- All three TypeScript examples compiled strictly against the emitted package declarations and
  actual `@misofm/engine` export mappings.
- Every repository-file link resolves.
- A fresh Astra MEDIUM adversarial review found one stale shipped-effect claim. Commit `30404121`
  removed the two undelivered effects; the same reviewer then returned PASS and confirmed the
  eight-effect list matches the generated catalog.

Decision: PASS. The README describes the current 0.4.0 product and its material boundaries without
changing code, package metadata, generated files, or artifacts.
