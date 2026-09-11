# Make npm verify mode reachable after an immutable version is published

## Problem and smallest closable slice

The existing `.github/workflows/npm-publish.yml` documents `verify` as recovery after registry propagation, but its shared `Re-smoke and checksum the exact prior-qualified archive` step runs `npm publish --dry-run --ignore-scripts "$archive"` for both `publish` and `verify`. With pinned npm 11.19.0, verify of an already-published immutable version fails there with `You cannot publish over the previously published versions` and never reaches registry convergence, fresh consumer imports, signature audit, or SLSA predicate checks. Engine 0.2.3 verify run `34640332474` demonstrates the defect; publish run `34640083474` first exposed the need for recovery after a successful publication outlived the workflow's 60-second registry window.

Deliver the smallest workflow correction that makes future verify-only recovery reachable. This is release tooling only. It does not alter or republish 0.2.3 and cannot retroactively change its provenance or failed workflow conclusions.

## Bounded change

- Keep unpacking, `sdk/test/package-tarball-smoke.mjs`, and SHA-1/SHA-256/SRI comparison against the downloaded qualification artifact in both `publish` and `verify` modes.
- Run `npm publish --dry-run --ignore-scripts "$archive"` only in `publish` mode, before the registry absence guard and the single real publish. Verify mode must never invoke any publish command, including dry run.
- Preserve exact-main/expected-SHA binding, qualification-run and named-artifact identity, immutable-version refusal before publish, OIDC-only publication, no retry after ambiguous publish, registry public/latest convergence, fresh registry consumer imports/enginectl, signature audit, exact SLSA subject/workflow/ref/resolved-dependency predicates, action pins, and evidence upload.
- Do not add a second release workflow, a token fallback, a republish path, a broader retry policy, or a new release framework.

## Objective gates

1. A hermetic workflow-level test or existing workflow parser exercise proves the mode matrix: qualify does not download/publish; publish re-smokes, checksums, dry-runs exactly once, checks absence, and may publish exactly once; verify re-smokes and checksums but contains no reachable `npm publish` command and proceeds to registry/consumer/provenance checks for an existing version. Red mutations that restore dry-run to verify or remove it from publish must fail.
2. Shell/YAML validation and existing npm trust-list/release workflow checks pass. The normalized workflow diff contains only the necessary mode condition/test change; all cryptographic and exact-SHA assertions remain byte-equivalent or demonstrably equivalent.
3. Exercise the corrected verification path without publishing: use a fixture/mock boundary or a future already-qualified existing release as permitted by the issue, prove an existing immutable version does not stop at package inspection, and prove registry/provenance failures still fail closed. Never point a test publish at the public package.
4. Sol adversarial review confirms verify is read-only, publish retains its pre-publication dry run and one-shot semantics, and no provenance gate weakened. Required CI passes before merge; synchronize and close the matching GitHub issue after upstream evidence.

## Limits

One implementation attempt and one Sol verdict. If a bounded hermetic mode discriminator cannot be added without creating a release framework, stop and rebrief rather than expanding scope. No SDK version bump, artifact rebuild, DSP/app/adapter change, npm publication, tag change, or reinterpretation of issue #753 evidence belongs here.
